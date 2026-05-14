#include <gpu/rho_guesser.h>

#include <cuda_runtime.h>

#include <cmath>
#include <cstdio>
#include <vector>

namespace {

constexpr int kThreadsPerBlock = 256;
constexpr int kMaxDemoAtoms = 128;
constexpr int kAtomTypeCount = 2;
constexpr double kTwoPi = 6.28318530717958647692;

struct DeviceComplex {
    double real;
    double imag;
};

struct AtomTypeParams {
    double charge;
    double gaussian_width;
};

__constant__ AtomTypeParams kAtomTypes[kAtomTypeCount];

bool checkCuda(cudaError_t status, const char* operation) {
    if (status == cudaSuccess) {
        return true;
    }

    std::fprintf(stderr, "CUDA rho guess %s failed: %s\n", operation, cudaGetErrorString(status));
    return false;
}

__device__ int centeredIndex(int index, int size) {
    return index <= size / 2 ? index : index - size;
}

__device__ double atomicFormFactor(int atom_type, double g2) {
    const AtomTypeParams params = kAtomTypes[atom_type];
    return params.charge * exp(-params.gaussian_width * g2);
}

__global__ void buildRhoGKernel(DeviceComplex* __restrict__ rho_g,
                                const double* __restrict__ atom_x,
                                const double* __restrict__ atom_y,
                                const double* __restrict__ atom_z,
                                const int* __restrict__ atom_type,
                                int atom_count,
                                int nx,
                                int ny,
                                int nz) {
    __shared__ double sh_x[kMaxDemoAtoms];
    __shared__ double sh_y[kMaxDemoAtoms];
    __shared__ double sh_z[kMaxDemoAtoms];
    __shared__ int sh_type[kMaxDemoAtoms];

    for (int atom_index = threadIdx.x; atom_index < atom_count; atom_index += blockDim.x) {
        sh_x[atom_index] = atom_x[atom_index];
        sh_y[atom_index] = atom_y[atom_index];
        sh_z[atom_index] = atom_z[atom_index];
        sh_type[atom_index] = atom_type[atom_index];
    }
    __syncthreads();

    const int point_count = nx * ny * nz;
    const int linear_index = blockIdx.x * blockDim.x + threadIdx.x;
    if (linear_index >= point_count) {
        return;
    }

    const int ix = linear_index % nx;
    const int iy = (linear_index / nx) % ny;
    const int iz = linear_index / (nx * ny);

    const double gx = kTwoPi * static_cast<double>(centeredIndex(ix, nx));
    const double gy = kTwoPi * static_cast<double>(centeredIndex(iy, ny));
    const double gz = kTwoPi * static_cast<double>(centeredIndex(iz, nz));
    const double g2 = gx * gx + gy * gy + gz * gz;

    double real = 0.0;
    double imag = 0.0;

    for (int atom_index = 0; atom_index < atom_count; ++atom_index) {
        const double amplitude = atomicFormFactor(sh_type[atom_index], g2);
        const double phase = gx * sh_x[atom_index] + gy * sh_y[atom_index] + gz * sh_z[atom_index];

        double sin_phase = 0.0;
        double cos_phase = 0.0;
        sincos(phase, &sin_phase, &cos_phase);

        real += amplitude * cos_phase;
        imag -= amplitude * sin_phase;
    }

    rho_g[linear_index] = {real, imag};
}

__global__ void reduceRhoMagnitudeKernel(const DeviceComplex* __restrict__ rho_g,
                                         double* __restrict__ partial_sums,
                                         int point_count) {
    __shared__ double sh_sum[kThreadsPerBlock];

    const int linear_index = blockIdx.x * blockDim.x + threadIdx.x;
    double local_sum = 0.0;
    if (linear_index < point_count) {
        const DeviceComplex value = rho_g[linear_index];
        local_sum = value.real * value.real + value.imag * value.imag;
    }

    sh_sum[threadIdx.x] = local_sum;
    __syncthreads();

    for (int stride = blockDim.x / 2; stride > 0; stride /= 2) {
        if (threadIdx.x < stride) {
            sh_sum[threadIdx.x] += sh_sum[threadIdx.x + stride];
        }
        __syncthreads();
    }

    if (threadIdx.x == 0) {
        partial_sums[blockIdx.x] = sh_sum[0];
    }
}

void fillDemoAtoms(std::vector<double>& x,
                   std::vector<double>& y,
                   std::vector<double>& z,
                   std::vector<int>& type) {
    constexpr int atoms_per_axis = 4;
    x.reserve(atoms_per_axis * atoms_per_axis * atoms_per_axis);
    y.reserve(atoms_per_axis * atoms_per_axis * atoms_per_axis);
    z.reserve(atoms_per_axis * atoms_per_axis * atoms_per_axis);
    type.reserve(atoms_per_axis * atoms_per_axis * atoms_per_axis);

    for (int iz = 0; iz < atoms_per_axis; ++iz) {
        for (int iy = 0; iy < atoms_per_axis; ++iy) {
            for (int ix = 0; ix < atoms_per_axis; ++ix) {
                x.push_back((static_cast<double>(ix) + 0.5) / atoms_per_axis);
                y.push_back((static_cast<double>(iy) + 0.5) / atoms_per_axis);
                z.push_back((static_cast<double>(iz) + 0.5) / atoms_per_axis);
                type.push_back((ix + iy + iz) % kAtomTypeCount);
            }
        }
    }
}

bool runRhoGuess(int nx, int ny, int nz) {
    std::vector<double> atom_x;
    std::vector<double> atom_y;
    std::vector<double> atom_z;
    std::vector<int> atom_type;
    fillDemoAtoms(atom_x, atom_y, atom_z, atom_type);

    const int atom_count = static_cast<int>(atom_x.size());
    if (atom_count > kMaxDemoAtoms) {
        std::fprintf(stderr, "CUDA rho guess failed: demo atom count exceeds shared-memory limit.\n");
        return false;
    }

    const int point_count = nx * ny * nz;
    const int block_count = (point_count + kThreadsPerBlock - 1) / kThreadsPerBlock;

    const AtomTypeParams atom_types[kAtomTypeCount] = {
        {1.0, 0.0040},
        {4.0, 0.0025},
    };

    double* device_x = nullptr;
    double* device_y = nullptr;
    double* device_z = nullptr;
    int* device_type = nullptr;
    DeviceComplex* device_rho_g = nullptr;
    double* device_partial_sums = nullptr;

    cudaEvent_t start_event = nullptr;
    cudaEvent_t stop_event = nullptr;

    const std::size_t atom_bytes = static_cast<std::size_t>(atom_count) * sizeof(double);
    const std::size_t type_bytes = static_cast<std::size_t>(atom_count) * sizeof(int);
    const std::size_t rho_bytes = static_cast<std::size_t>(point_count) * sizeof(DeviceComplex);
    const std::size_t partial_bytes = static_cast<std::size_t>(block_count) * sizeof(double);

    bool ok = checkCuda(cudaMemcpyToSymbol(kAtomTypes, atom_types, sizeof(atom_types)), "copy atom type constants") &&
              checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_x), atom_bytes), "allocate atom_x") &&
              checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_y), atom_bytes), "allocate atom_y") &&
              checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_z), atom_bytes), "allocate atom_z") &&
              checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_type), type_bytes), "allocate atom_type") &&
              checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_rho_g), rho_bytes), "allocate rho_g") &&
              checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_partial_sums), partial_bytes), "allocate reductions") &&
              checkCuda(cudaMemcpy(device_x, atom_x.data(), atom_bytes, cudaMemcpyHostToDevice), "copy atom_x") &&
              checkCuda(cudaMemcpy(device_y, atom_y.data(), atom_bytes, cudaMemcpyHostToDevice), "copy atom_y") &&
              checkCuda(cudaMemcpy(device_z, atom_z.data(), atom_bytes, cudaMemcpyHostToDevice), "copy atom_z") &&
              checkCuda(cudaMemcpy(device_type, atom_type.data(), type_bytes, cudaMemcpyHostToDevice), "copy atom_type") &&
              checkCuda(cudaEventCreate(&start_event), "create start event") &&
              checkCuda(cudaEventCreate(&stop_event), "create stop event");

    if (ok) {
        ok = checkCuda(cudaEventRecord(start_event), "record rho start");
    }

    if (ok) {
        buildRhoGKernel<<<block_count, kThreadsPerBlock>>>(
            device_rho_g, device_x, device_y, device_z, device_type, atom_count, nx, ny, nz);
        ok = checkCuda(cudaGetLastError(), "launch rho(G) kernel");
    }

    if (ok) {
        reduceRhoMagnitudeKernel<<<block_count, kThreadsPerBlock>>>(device_rho_g, device_partial_sums, point_count);
        ok = checkCuda(cudaGetLastError(), "launch rho checksum kernel");
    }

    if (ok) {
        ok = checkCuda(cudaEventRecord(stop_event), "record rho stop") &&
             checkCuda(cudaEventSynchronize(stop_event), "synchronize rho stop");
    }

    float elapsed_ms = 0.0f;
    std::vector<double> partial_sums(block_count, 0.0);
    if (ok) {
        ok = checkCuda(cudaEventElapsedTime(&elapsed_ms, start_event, stop_event), "measure rho time") &&
             checkCuda(cudaMemcpy(partial_sums.data(), device_partial_sums, partial_bytes, cudaMemcpyDeviceToHost),
                       "copy rho checksum");
    }

    cudaEventDestroy(start_event);
    cudaEventDestroy(stop_event);
    cudaFree(device_x);
    cudaFree(device_y);
    cudaFree(device_z);
    cudaFree(device_type);
    cudaFree(device_rho_g);
    cudaFree(device_partial_sums);

    if (!ok) {
        return false;
    }

    double magnitude_sum = 0.0;
    for (double partial_sum : partial_sums) {
        magnitude_sum += partial_sum;
    }

    const double points_per_ms = static_cast<double>(point_count) / elapsed_ms;
    std::printf("CUDA rho guess demo:\n");
    std::printf("  Method: one CUDA thread per reciprocal-space G point, no atomics\n");
    std::printf("  Grid: %d x %d x %d = %d G points\n", nx, ny, nz, point_count);
    std::printf("  Demo atoms: %d across %d atom types\n", atom_count, kAtomTypeCount);
    std::printf("  Kernel+checksum time: %.3f ms\n", elapsed_ms);
    std::printf("  Throughput: %.2f million G-points/s\n", points_per_ms / 1000.0);
    std::printf("  rho(G) magnitude checksum: %.8e\n", magnitude_sum);
    return true;
}

}  // namespace

bool runRhoGuessDemo() {
    constexpr int nx = 64;
    constexpr int ny = 64;
    constexpr int nz = 64;
    return runRhoGuess(nx, ny, nz);
}
