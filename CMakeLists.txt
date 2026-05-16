cmake_minimum_required(VERSION 3.18)
project(machine LANGUAGES CXX CUDA)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CUDA_STANDARD 17)
set(CMAKE_CUDA_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT DEFINED CMAKE_CUDA_ARCHITECTURES)
    set(CMAKE_CUDA_ARCHITECTURES 86 89)
endif()

find_package(SQLite3 REQUIRED)
find_package(CURL REQUIRED)
find_package(CLI11 CONFIG REQUIRED)
find_package(CUDAToolkit REQUIRED)
find_package(Arrow CONFIG REQUIRED)
find_package(Parquet CONFIG REQUIRED)

set(NVCOMP_ROOT "/usr/local/lib/python3.10/dist-packages/nvidia/libnvcomp" CACHE PATH "nvCOMP install root")
find_path(NVCOMP_INCLUDE_DIR nvcomp.hpp PATHS "${NVCOMP_ROOT}/include" REQUIRED)
find_library(NVCOMP_LIBRARY NAMES nvcomp libnvcomp.so.5 PATHS "${NVCOMP_ROOT}/lib64" REQUIRED)

file(GLOB_RECURSE MACHINE_SOURCES CONFIGURE_DEPENDS src/*.cpp)
file(GLOB_RECURSE MACHINE_CUDA_SOURCES CONFIGURE_DEPENDS src/*.cu)

add_executable(machine ${MACHINE_SOURCES} ${MACHINE_CUDA_SOURCES})
target_include_directories(machine PRIVATE src)
target_include_directories(machine PRIVATE ${NVCOMP_INCLUDE_DIR})
target_link_libraries(machine PRIVATE
    SQLite::SQLite3
    CURL::libcurl
    CLI11::CLI11
    CUDA::cudart
    Arrow::arrow_shared
    Parquet::parquet_shared
    ${NVCOMP_LIBRARY}
)
set_target_properties(machine PROPERTIES CUDA_SEPARABLE_COMPILATION ON)
