

file(GLOB_RECURSE TEST_SOURCES "days/*.cpp")
add_executable(runTests ${TEST_SOURCES})

# Link GoogleTest framework and your main application
target_link_libraries(runTests gtest gtest_main YourLibraryName)

target_include_directories(runTests PRIVATE ${CMAKE_SOURCE_DIR}/src)

# Add a test
add_test(NAME ExampleTest COMMAND runTests)
