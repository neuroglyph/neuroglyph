# SPDX-License-Identifier: Apache-2.0
cmake_minimum_required(VERSION 3.10)
project(git-mind VERSION 0.1.0 LANGUAGES C)

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

# Include directories
include_directories(include)

# Main executable source files (no test code!)
set(SOURCES
    src/git-mind.c
    src/link.c
    src/sha1.c
    src/path.c
    src/check.c
    src/status.c
    src/traverse.c
    src/main.c
)

# Main executable - pure C, no external dependencies
add_executable(git-mind ${SOURCES})

# Static linking (optional)
if(BUILD_STATIC)
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static")
endif()

# Installation
install(TARGETS git-mind DESTINATION bin)
install(FILES include/git-mind.h DESTINATION include)

# Tests (optional) - completely separate from main binary
option(BUILD_TESTS "Build test suite" OFF)
if(BUILD_TESTS)
    enable_testing()
    
    # Find CMocka only when building tests
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(CMOCKA REQUIRED cmocka)
    
    # Test executables are built separately
    add_subdirectory(tests)
endif()