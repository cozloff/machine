cmake_minimum_required(VERSION 3.10)
project(machine)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(SQLite3 REQUIRED)
find_package(CURL REQUIRED)

file(GLOB_RECURSE MACHINE_SOURCES CONFIGURE_DEPENDS src/*.cpp)

add_executable(machine ${MACHINE_SOURCES})
target_include_directories(machine PRIVATE src)
target_link_libraries(machine PRIVATE SQLite::SQLite3 CURL::libcurl)
