cmake_minimum_required(VERSION 3.12)

set(PROJECT_NAME AdventOfCode2018)
project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
# set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE INTERNAL "")
# set(CMAKE_CXX_CLANG_TIDY clang-tidy -checks=-*,readability-*)

add_subdirectory(src)

include(FetchContent)

FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/tags/v1.14.0.zip
)

set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

FetchContent_MakeAvailable(googletest)

enable_testing()
add_subdirectory(test)

# option(test "Build all tests." OFF) # Makes boolean 'test' available.

# https://github.com/dacorvo/gtest-cmake-example

#
#   file(GLOB_RECURSE TEST_SOURCES "test/days/*.cpp")
#
#   add_executable(runUnitTests ${TEST_SOURCES})
#   target_link_libraries(
#     runUnitTests
#     GTest::gtest_main
#   )
#
#   include(GoogleTest)
#   gtest_discover_tests(runUnitTests)
# endif()
