cmake_minimum_required(VERSION 3.10)
project(machine)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(SQLite3 REQUIRED)

add_executable(machine main.cpp)
target_link_libraries(machine PRIVATE SQLite::SQLite3)
