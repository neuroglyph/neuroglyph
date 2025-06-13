# SPDX-License-Identifier: Apache-2.0
cmake_minimum_required(VERSION 3.10)
project(gitmind VERSION 0.1.0 LANGUAGES C)

# C99 standard
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Compiler flags
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall -Wextra -pedantic")
set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -g -O0")
set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} -O3")

# Sanitizers for debug builds
if(CMAKE_BUILD_TYPE STREQUAL "Debug" AND NOT APPLE)
    set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} -fsanitize=address,undefined")
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} -fsanitize=address,undefined")
endif()

# Find OpenSSL for SHA1
find_package(OpenSSL REQUIRED)

# Include directories
include_directories(include)

# Source files
set(SOURCES
    src/gitmind.c
    src/link.c
    src/sha1.c
    src/main.c
)

# Executable
add_executable(gitmind ${SOURCES})

# Link libraries
target_link_libraries(gitmind OpenSSL::Crypto)

# Static linking (optional)
if(BUILD_STATIC)
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static")
endif()

# Installation
install(TARGETS gitmind DESTINATION bin)
install(FILES include/gitmind.h DESTINATION include)

# Tests (optional)
if(BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()