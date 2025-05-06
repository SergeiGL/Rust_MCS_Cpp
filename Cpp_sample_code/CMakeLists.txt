cmake_minimum_required(VERSION 3.15)
project(your_program)

# Specify C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add include directories
include_directories(${CMAKE_SOURCE_DIR}/include)

# Find the Rust library
find_library(RUST_MCS_LIB
    NAMES Rust_MCS_Cpp
    PATHS ${CMAKE_SOURCE_DIR}/lib
    REQUIRED
)

# Add executable
add_executable(your_program src/main.cpp)

# Link against the Rust library
target_link_libraries(your_program PRIVATE ${RUST_MCS_LIB})

# Copy the DLL to the output directory
add_custom_command(TARGET your_program POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
    "${CMAKE_SOURCE_DIR}/lib/Rust_MCS_Cpp.dll"
    $<TARGET_FILE_DIR:your_program>
)