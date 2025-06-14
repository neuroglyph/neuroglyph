// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#ifndef GITMIND_H
#define GITMIND_H

#include <stddef.h>
#include <time.h>

#ifdef __cplusplus
extern "C" {
#endif

// Version
#define GITMIND_VERSION_MAJOR 0
#define GITMIND_VERSION_MINOR 1
#define GITMIND_VERSION_PATCH 0

// Error codes
typedef enum {
    GM_OK = 0,
    GM_ERR_NOT_REPO = -1,
    GM_ERR_NOT_FOUND = -2,
    GM_ERR_IO = -3,
    GM_ERR_GIT = -4,
    GM_ERR_MEMORY = -5,
    GM_ERR_INVALID_ARG = -6,
    GM_ERR_PATH_TOO_LONG = -7,
    GM_ERR_ALREADY_EXISTS = -8
} gm_error_t;

// Error message constants
#define ERR_MSG_NOT_REPO "Not a git repository: %s"
#define ERR_MSG_NOT_FOUND "Not found: %s"
#define ERR_MSG_IO_ERROR "I/O error: %s"
#define ERR_MSG_GIT_FAILED "Git operation failed: %s"
#define ERR_MSG_MEMORY "Memory allocation failed"
#define ERR_MSG_INVALID_ARG "Invalid argument: %s"
#define ERR_MSG_PATH_TOO_LONG "Path too long: %s"
#define ERR_MSG_ALREADY_EXISTS "Already exists: %s"
#define ERR_MSG_EMPTY_PATH "Empty path"
#define ERR_MSG_ABSOLUTE_PATH "Absolute paths not allowed in links: %s"
#define ERR_MSG_PATH_TRAVERSAL "Path traversal not allowed: %s"
#define ERR_MSG_LINK_NOT_FOUND "Link not found"
#define ERR_MSG_DIR_CREATE_FAILED "Failed to create directory %s: %s"
#define ERR_MSG_FILE_CREATE_FAILED "Failed to create file: %s"
#define ERR_MSG_FILE_WRITE_FAILED "Failed to write file"
#define ERR_MSG_LINK_CONTENT_TOO_LONG "Link content too long"
#define ERR_MSG_CWD_FAILED "Failed to get current directory"
#define ERR_MSG_NULL_POINTER "NULL pointer provided for %s"

// CLI error messages
#define ERR_MSG_LINK_REQUIRES_ARGS "Error: link requires source and target arguments\n"
#define ERR_MSG_UNLINK_REQUIRES_ARGS "Error: unlink requires source and target arguments\n"
#define ERR_MSG_UNKNOWN_COMMAND "Error: Unknown command '%s'\n"

// Status messages
#define MSG_INIT_SUCCESS "Initialized gitmind in current repository\n"
#define MSG_LINK_CREATED "Created link: %s -> %s (%s)\n"
#define MSG_LINK_REMOVED "Removed link: %s -> %s\n"
#define MSG_NO_LINKS "No links found\n"
#define MSG_ALL_LINKS_VALID "All links are valid\n"
#define MSG_BROKEN_LINKS_FOUND "Found %d broken link%s\n"
#define MSG_BROKEN_LINKS_REMOVED "Removed %d broken link%s\n"
#define MSG_RUN_CHECK_FIX "Run 'gitmind check --fix' to remove them\n"
#define MSG_NOT_INITIALIZED "GitMind: Not initialized (run 'gitmind init')\n"

// Version info
#define MSG_VERSION_FORMAT "gitmind version %s\n"

// Buffer sizes - centralized for maintainability
#define GM_MAX_PATH 4096
#define GM_MAX_TYPE 64
#define GM_MAX_LINK_CONTENT 8192
#define GM_MAX_COMMAND 8192
#define GM_ERROR_BUFFER_SIZE 256

// SHA constants
#define GM_SHA1_SIZE 20
#define GM_SHA1_STRING_SIZE 41

// Default values
#define GM_DEFAULT_LINK_TYPE "REFERENCES"
#define GM_LINKS_DIR ".gitmind/links"
#define GM_LINK_EXTENSION ".link"

// Link structure
typedef struct {
    char type[GM_MAX_TYPE];
    char source[GM_MAX_PATH];
    char target[GM_MAX_PATH];
    time_t timestamp;
} gm_link_t;

// Link collection
typedef struct {
    gm_link_t* links;
    size_t count;
    size_t capacity;
} gm_link_set_t;

// Repository handle
typedef struct gm_repo gm_repo;

// Traversal constants
#define GM_MAX_DEPTH 10
#define GM_DEFAULT_DEPTH 1

// Traversal formats
typedef enum {
    GM_FORMAT_TREE = 0,
    GM_FORMAT_LIST = 1
} gm_format_t;

// Traversal node
typedef struct {
    char path[GM_MAX_PATH];
    int depth;
    char parent[GM_MAX_PATH];
} gm_traverse_node_t;

// Traversal result
typedef struct {
    gm_traverse_node_t* nodes;
    size_t count;
    size_t capacity;
    int direct_count;
    int total_count;
} gm_traverse_result_t;

// Core operations
int gm_init(const char* repo_path);
int gm_link_create(const char* source, const char* target, const char* type);
int gm_link_list(gm_link_set_t** set, const char* filter_source, const char* filter_target);
int gm_link_unlink(const char* source, const char* target);
int gm_link_unlink_all(const char* source);
int gm_link_check(int fix, int* broken_count);
int gm_status(void);
int gm_traverse(const char* start_node, int depth, gm_format_t format, gm_traverse_result_t** result);

// Link set operations
gm_link_set_t* gm_link_set_new(void);
void gm_link_set_free(gm_link_set_t* set);
int gm_link_set_add(gm_link_set_t* set, const gm_link_t* link);

// Traverse result operations
gm_traverse_result_t* gm_traverse_result_new(void);
void gm_traverse_result_free(gm_traverse_result_t* result);
int gm_traverse_result_add(gm_traverse_result_t* result, const gm_traverse_node_t* node);
void gm_traverse_print_tree(const gm_traverse_result_t* result, const char* start_node);
void gm_traverse_print_list(const gm_traverse_result_t* result, const char* start_node);

// Error handling
const char* gm_last_error(void);
const char* gm_error_string(int error_code);

// Utilities
int gm_sha1_string(const char* content, char* out_sha);
int gm_normalize_path(const char* path, char* out_normalized);
int gm_path_join(char* dest, size_t dest_size, const char* dir, const char* file);
int gm_validate_link_path(const char* path);

// Internal - not part of public API
void gm_set_error(const char* fmt, ...);

// Version info
const char* gm_version_string(void);

#ifdef __cplusplus
}
#endif

#endif // GITMIND_H
