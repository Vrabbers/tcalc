include(FetchContent)
FetchContent_Declare(
    googletest
    URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()

add_executable(tcalc_tests
    test-simple-operations.cpp
    test-errors.cpp
)
target_link_libraries(tcalc_tests
    libtcalc
    GTest::gtest_main
)
target_include_directories(tcalc_tests PRIVATE
    ../libtcalc
)

include(GoogleTest)
gtest_discover_tests(tcalc_tests)
