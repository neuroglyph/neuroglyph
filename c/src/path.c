// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
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

// Check if a path component is exactly ".."
static int is_parent_ref(const char* start, const char* end) {
    size_t len = end - start;
    return len == 2 && start[0] == '.' && start[1] == '.';
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
    
    // Windows-style absolute paths (C:\, D:\, etc.)
    if (strlen(path) >= 3 && path[1] == ':' && (path[2] == '\\' || path[2] == '/')) {
        gm_set_error("Absolute paths not allowed in links: %s", path);
        return GM_ERR_INVALID_ARG;
    }
    
    // Length check
    if (strlen(path) >= GM_MAX_PATH) {
        gm_set_error("Path too long: %s", path);
        return GM_ERR_PATH_TOO_LONG;
    }
    
    // Parse path components to detect ".."
    const char* p = path;
    const char* component_start = p;
    
    while (*p) {
        if (*p == '/' || *p == '\\') {
            // Check if this component is ".."
            if (is_parent_ref(component_start, p)) {
                gm_set_error("Path traversal not allowed: %s", path);
                return GM_ERR_INVALID_ARG;
            }
            // Skip the separator and start next component
            p++;
            component_start = p;
        } else {
            p++;
        }
    }
    
    // Check the last component
    if (is_parent_ref(component_start, p)) {
        gm_set_error("Path traversal not allowed: %s", path);
        return GM_ERR_INVALID_ARG;
    }
    
    return GM_OK;
}