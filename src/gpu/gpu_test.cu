#include <gpu/gpu_test.h>

#include <cuda_runtime.h>

#include <cstddef>
#include <cstdio>

namespace {

bool checkCuda(cudaError_t status, const char* operation) {
    if (status == cudaSuccess) {
        return true;
    }

    std::fprintf(stderr, "CUDA %s failed: %s\n", operation, cudaGetErrorString(status));
    return false;
}

double bytesToGiB(std::size_t bytes) {
    return static_cast<double>(bytes) / (1024.0 * 1024.0 * 1024.0);
}

double bytesToKiB(std::size_t bytes) {
    return static_cast<double>(bytes) / 1024.0;
}

double theoreticalMemoryBandwidthGbps(const cudaDeviceProp& prop) {
    const double memory_clock_hz = static_cast<double>(prop.memoryClockRate) * 1000.0;
    const double bus_width_bytes = static_cast<double>(prop.memoryBusWidth) / 8.0;
    return 2.0 * memory_clock_hz * bus_width_bytes / 1.0e9;
}

void printDeviceSpecs(int device_index, const cudaDeviceProp& prop) {
    std::printf("CUDA GPU %d: %s\n", device_index, prop.name);
    std::printf("  Compute capability: %d.%d\n", prop.major, prop.minor);
    std::printf("  SMs: %d\n", prop.multiProcessorCount);
    std::printf("  Max blocks per SM: %d\n", prop.maxBlocksPerMultiProcessor);
    std::printf("  Max threads per SM: %d\n", prop.maxThreadsPerMultiProcessor);
    std::printf("  Max threads per block: %d\n", prop.maxThreadsPerBlock);
    std::printf("  Warp size: %d\n", prop.warpSize);
    std::printf("  Max block dimensions: %d x %d x %d threads\n",
                prop.maxThreadsDim[0],
                prop.maxThreadsDim[1],
                prop.maxThreadsDim[2]);
    std::printf("  Max grid dimensions: %d x %d x %d blocks\n",
                prop.maxGridSize[0],
                prop.maxGridSize[1],
                prop.maxGridSize[2]);
    std::printf("  Global memory: %.2f GiB\n", bytesToGiB(prop.totalGlobalMem));
    std::printf("  Memory type: not exposed by CUDA runtime API; reported here as device global memory\n");
    std::printf("  Memory bus width: %d-bit\n", prop.memoryBusWidth);
    std::printf("  Memory clock: %.2f GHz effective data-rate source clock\n",
                static_cast<double>(prop.memoryClockRate) / 1000000.0);
    std::printf("  Theoretical memory bandwidth: %.2f GB/s\n", theoreticalMemoryBandwidthGbps(prop));
    std::printf("  L2 cache: %.2f KiB\n", bytesToKiB(prop.l2CacheSize));
    std::printf("  Shared memory per block: %.2f KiB\n", bytesToKiB(prop.sharedMemPerBlock));
    std::printf("  Shared memory per SM: %.2f KiB\n", bytesToKiB(prop.sharedMemPerMultiprocessor));
    std::printf("  Registers per block: %d\n", prop.regsPerBlock);
    std::printf("  Registers per SM: %d\n", prop.regsPerMultiprocessor);
    std::printf("  Core clock: %.2f GHz\n", static_cast<double>(prop.clockRate) / 1000000.0);
    std::printf("  Concurrent kernels: %s\n", prop.concurrentKernels ? "yes" : "no");
    std::printf("  Async copy engines: %d\n", prop.asyncEngineCount);
    std::printf("  Unified addressing: %s\n", prop.unifiedAddressing ? "yes" : "no");
    std::printf("  Managed memory: %s\n", prop.managedMemory ? "yes" : "no");
}

}  // namespace

bool runGpuTest() {
    int device_count = 0;
    if (!checkCuda(cudaGetDeviceCount(&device_count), "device count")) {
        return false;
    }

    if (device_count == 0) {
        std::fprintf(stderr, "CUDA GPU test failed: no CUDA-capable device found.\n");
        return false;
    }

    std::printf("CUDA device report: %d CUDA-capable device%s found\n",
                device_count,
                device_count == 1 ? "" : "s");

    for (int device_index = 0; device_index < device_count; ++device_index) {
        cudaDeviceProp prop = {};
        if (!checkCuda(cudaGetDeviceProperties(&prop, device_index), "device properties")) {
            return false;
        }

        printDeviceSpecs(device_index, prop);
    }

    return true;
}
