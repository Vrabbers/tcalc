cmake_minimum_required(VERSION 3.21.0)

project(tcalc VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

option(BUILD_TESTS "Whether to build unit tests" ON)

if (MSVC)
    add_compile_options(/W4 /utf-8)
else() 
    add_compile_options(-Wall -Wextra)
endif()

add_subdirectory(libtcalc)
add_subdirectory(tcalc_console)

if(BUILD_TESTS)
    add_subdirectory(tests)
endif()