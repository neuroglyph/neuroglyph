# SPDX-License-Identifier: Apache-2.0
# Tests are completely separate from the main binary

# Test source files (excluding main.c!)
set(TEST_SOURCES
    ../src/gitmind.c
    ../src/link.c
    ../src/sha1.c
    ../src/path.c
    ../src/check.c
    ../src/status.c
    ../src/traverse.c
)

# Path validation tests
add_executable(test_path_validation
    test_path_validation.c
    ${TEST_SOURCES}
)
target_include_directories(test_path_validation PRIVATE ../include ${CMOCKA_INCLUDE_DIRS})
target_link_libraries(test_path_validation ${CMOCKA_LIBRARIES})
target_compile_options(test_path_validation PRIVATE ${CMOCKA_CFLAGS_OTHER})
add_test(NAME path_validation COMMAND test_path_validation)

# Memory leak tests (requires linker wrapping)
add_executable(test_memory_leaks
    test_memory_leaks.c
    ${TEST_SOURCES}
)
target_include_directories(test_memory_leaks PRIVATE ../include ${CMOCKA_INCLUDE_DIRS})
target_link_libraries(test_memory_leaks ${CMOCKA_LIBRARIES})
target_compile_options(test_memory_leaks PRIVATE ${CMOCKA_CFLAGS_OTHER})
# Wrap malloc/free for leak tracking
target_link_options(test_memory_leaks PRIVATE 
    -Wl,--wrap=malloc
    -Wl,--wrap=free
)
add_test(NAME memory_leaks COMMAND test_memory_leaks)

# Error consistency tests
add_executable(test_error_consistency
    test_error_consistency.c
    ${TEST_SOURCES}
)
target_include_directories(test_error_consistency PRIVATE ../include ${CMOCKA_INCLUDE_DIRS})
target_link_libraries(test_error_consistency ${CMOCKA_LIBRARIES})
target_compile_options(test_error_consistency PRIVATE ${CMOCKA_CFLAGS_OTHER})
add_test(NAME error_consistency COMMAND test_error_consistency)