// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
#include "errors.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <limits.h>

// Safe path joining with bounds checking
int gm_path_join(char* dest, size_t dest_size, const char* dir, const char* file) {
    int ret = snprintf(dest, dest_size, "%s/%s", dir, file);
    if (ret < 0 || (size_t)ret >= dest_size) {
        gm_set_error(ERR_MSG_PATH_TOO_LONG, dir);
        return GM_ERR_PATH_TOO_LONG;
    }
    return GM_OK;
}

// Normalize path (remove ./, ../, trailing slashes)
int gm_normalize_path(const char* path, char* out_normalized) {
    if (!path || !out_normalized) {
        return GM_ERR_INVALID_ARG;
    }
    
    // For now, just copy - TODO: implement proper normalization
    strncpy(out_normalized, path, GM_MAX_PATH - 1);
    out_normalized[GM_MAX_PATH - 1] = '\0';
    
    // Remove trailing slash if present
    size_t len = strlen(out_normalized);
    if (len > 1 && out_normalized[len - 1] == '/') {
        out_normalized[len - 1] = '\0';
    }
    
    return GM_OK;
}

// Validate path is safe (no .., no absolute paths for links)
int gm_validate_link_path(const char* path) {
    if (!path || *path == '\0') {
        gm_set_error("Empty path");
        return GM_ERR_INVALID_ARG;
    }
    
    // No absolute paths in links
    if (path[0] == '/') {
        gm_set_error("Absolute paths not allowed in links: %s", path);
        return GM_ERR_INVALID_ARG;
    }
    
    // No .. components
    if (strstr(path, "..")) {
        gm_set_error("Path traversal not allowed: %s", path);
        return GM_ERR_INVALID_ARG;
    }
    
    // Length check
    if (strlen(path) >= GM_MAX_PATH) {
        gm_set_error("Path too long: %s", path);
        return GM_ERR_PATH_TOO_LONG;
    }
    
    return GM_OK;
}