cmake_minimum_required(VERSION 3.10)
project(machine)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(SQLite3 REQUIRED)
find_package(CURL REQUIRED)

add_executable(machine src/main.cpp src/pubchem.cpp)
target_link_libraries(machine PRIVATE SQLite::SQLite3 CURL::libcurl)
