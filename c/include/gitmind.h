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

// Core operations
int gm_init(const char* repo_path);
int gm_link_create(const char* source, const char* target, const char* type);
int gm_link_list(gm_link_set_t** set, const char* filter_source, const char* filter_target);
int gm_link_unlink(const char* source, const char* target);
int gm_link_unlink_all(const char* source);
int gm_link_check(int fix, int* broken_count);
int gm_status(void);

// Link set operations
gm_link_set_t* gm_link_set_new(void);
void gm_link_set_free(gm_link_set_t* set);
int gm_link_set_add(gm_link_set_t* set, const gm_link_t* link);

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