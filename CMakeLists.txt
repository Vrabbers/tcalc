cmake_minimum_required(VERSION 3.10)

project(tcalc)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(SOURCES src/string_reader.cpp)

add_executable(tcalc src/main.cpp ${SOURCES})
target_compile_options(tcalc PRIVATE -Wall -Wextra -Wpedantic)

