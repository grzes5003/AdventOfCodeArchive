
file(GLOB_RECURSE SOURCES "days/*.cpp" "util/*.cpp" "main.cpp")

add_library(YourLibraryName STATIC ${SOURCES})

# Specify include directories for your library
target_include_directories(YourLibraryName PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

add_executable(aoc2018 ${SOURCES})
target_link_libraries(aoc2018 YourLibraryName)
