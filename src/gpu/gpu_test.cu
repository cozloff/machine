#include <gpu/gpu_test.h>

#include <cuda_runtime.h>

#include <cstdio>

namespace {

__global__ void addKernel(const int* a, const int* b, int* c) {
    const int index = threadIdx.x;
    c[index] = a[index] + b[index];
}

bool checkCuda(cudaError_t status, const char* operation) {
    if (status == cudaSuccess) {
        return true;
    }

    std::fprintf(stderr, "CUDA %s failed: %s\n", operation, cudaGetErrorString(status));
    return false;
}

}  // namespace

bool runGpuTest() {
    constexpr int value_count = 4;
    constexpr int bytes = value_count * static_cast<int>(sizeof(int));

    const int host_a[value_count] = {1, 2, 3, 4};
    const int host_b[value_count] = {10, 20, 30, 40};
    int host_c[value_count] = {};

    int* device_a = nullptr;
    int* device_b = nullptr;
    int* device_c = nullptr;

    int device_count = 0;
    if (!checkCuda(cudaGetDeviceCount(&device_count), "device count")) {
        return false;
    }

    if (device_count == 0) {
        std::fprintf(stderr, "CUDA GPU test failed: no CUDA-capable device found.\n");
        return false;
    }

    if (!checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_a), bytes), "allocate device_a") ||
        !checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_b), bytes), "allocate device_b") ||
        !checkCuda(cudaMalloc(reinterpret_cast<void**>(&device_c), bytes), "allocate device_c")) {
        cudaFree(device_a);
        cudaFree(device_b);
        cudaFree(device_c);
        return false;
    }

    const bool copied =
        checkCuda(cudaMemcpy(device_a, host_a, bytes, cudaMemcpyHostToDevice), "copy host_a") &&
        checkCuda(cudaMemcpy(device_b, host_b, bytes, cudaMemcpyHostToDevice), "copy host_b");

    if (!copied) {
        cudaFree(device_a);
        cudaFree(device_b);
        cudaFree(device_c);
        return false;
    }

    addKernel<<<1, value_count>>>(device_a, device_b, device_c);

    const bool kernel_ok =
        checkCuda(cudaGetLastError(), "launch addKernel") &&
        checkCuda(cudaDeviceSynchronize(), "synchronize addKernel") &&
        checkCuda(cudaMemcpy(host_c, device_c, bytes, cudaMemcpyDeviceToHost), "copy result");

    cudaFree(device_a);
    cudaFree(device_b);
    cudaFree(device_c);

    if (!kernel_ok) {
        return false;
    }

    for (int i = 0; i < value_count; ++i) {
        if (host_c[i] != host_a[i] + host_b[i]) {
            std::fprintf(stderr, "CUDA GPU test failed: unexpected result at %d.\n", i);
            return false;
        }
    }

    std::printf("CUDA GPU test passed: [%d, %d, %d, %d]\n", host_c[0], host_c[1], host_c[2], host_c[3]);
    return true;
}
