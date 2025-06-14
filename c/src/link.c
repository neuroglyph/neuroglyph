// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>

// Forward declarations are already in gitmind.h, no need to redeclare

// Build link content string
static int build_link_content(const gm_link_t* link, char* buffer, size_t size) {
    int len = snprintf(buffer, size, "%s: %s -> %s  # ts:%ld\n",
        link->type, link->source, link->target, (long)link->timestamp);
    
    if (len < 0 || (size_t)len >= size) {
        gm_set_error("Link content too long");
        return GM_ERR_PATH_TOO_LONG;
    }
    
    return GM_OK;
}

// Get current working directory
static int get_cwd(char* buffer, size_t size) {
    if (!getcwd(buffer, size)) {
        gm_set_error("Failed to get current directory");
        return GM_ERR_IO;
    }
    return GM_OK;
}

// Create a link
int gm_link_create(const char* source, const char* target, const char* type) {
    if (!source || !target || !type) {
        gm_set_error("Invalid arguments");
        return GM_ERR_INVALID_ARG;
    }
    
    // Validate paths
    int ret = gm_validate_link_path(source);
    if (ret != GM_OK) return ret;
    
    ret = gm_validate_link_path(target);
    if (ret != GM_OK) return ret;
    
    // Build link structure
    gm_link_t link;
    snprintf(link.type, GM_MAX_TYPE, "%s", type);
    snprintf(link.source, GM_MAX_PATH, "%s", source);
    snprintf(link.target, GM_MAX_PATH, "%s", target);
    link.timestamp = time(NULL);
    
    // Build content
    char content[GM_MAX_LINK_CONTENT];
    ret = build_link_content(&link, content, sizeof(content));
    if (ret != GM_OK) return ret;
    
    // Compute SHA1
    char sha[GM_SHA1_STRING_SIZE];
    ret = gm_sha1_string(content, sha);
    if (ret != GM_OK) return ret;
    
    // Build filename
    char cwd[GM_MAX_PATH];
    ret = get_cwd(cwd, sizeof(cwd));
    if (ret != GM_OK) return ret;
    
    char links_dir[GM_MAX_PATH];
    char filename[GM_MAX_PATH];
    char link_name[GM_MAX_PATH];
    
    // Build path safely
    if (gm_path_join(links_dir, sizeof(links_dir), cwd, ".gitmind/links") != GM_OK) {
        gm_set_error("Path too long");
        return GM_ERR_PATH_TOO_LONG;
    }
    
    snprintf(link_name, sizeof(link_name), "%s.link", sha);
    if (gm_path_join(filename, sizeof(filename), links_dir, link_name) != GM_OK) {
        gm_set_error("Path too long");
        return GM_ERR_PATH_TOO_LONG;
    }
    
    // Check if link already exists
    struct stat st;
    if (stat(filename, &st) == 0) {
        return GM_OK; // Link already exists, idempotent
    }
    
    // Write file
    FILE* f = fopen(filename, "w");
    if (!f) {
        gm_set_error("Failed to create link file: %s", filename);
        return GM_ERR_IO;
    }
    
    if (fputs(content, f) == EOF) {
        fclose(f);
        gm_set_error("Failed to write link file");
        return GM_ERR_IO;
    }
    
    fclose(f);
    
    // Git add the file (optional - we don't care if it fails)
    char cmd[GM_MAX_PATH * 2];
    snprintf(cmd, sizeof(cmd), "git add %s 2>/dev/null", filename);
    if (system(cmd) != 0) {
        // Ignore failure - git add is optional
    }
    
    return GM_OK;
}

// Parse link file
static int parse_link_file(const char* filename, gm_link_t* link) {
    FILE* f = fopen(filename, "r");
    if (!f) {
        return GM_ERR_IO;
    }
    
    char line[8192];
    if (!fgets(line, sizeof(line), f)) {
        fclose(f);
        return GM_ERR_IO;
    }
    fclose(f);
    
    // Remove newline
    char* nl = strchr(line, '\n');
    if (nl) *nl = '\0';
    
    // Parse format: "TYPE: source -> target  # ts:12345"
    char* colon = strchr(line, ':');
    if (!colon) return GM_ERR_IO;
    
    // Extract type
    *colon = '\0';
    snprintf(link->type, GM_MAX_TYPE, "%s", line);
    
    // Find arrow
    char* arrow = strstr(colon + 1, " -> ");
    if (!arrow) return GM_ERR_IO;
    
    // Extract source (skip leading space)
    *arrow = '\0';
    char* source_start = colon + 1;
    while (*source_start == ' ') source_start++;
    snprintf(link->source, GM_MAX_PATH, "%s", source_start);
    
    // Find timestamp marker
    char* ts_marker = strstr(arrow + 4, "  # ts:");
    if (!ts_marker) return GM_ERR_IO;
    
    // Extract target
    *ts_marker = '\0';
    snprintf(link->target, GM_MAX_PATH, "%s", arrow + 4);
    
    // Extract timestamp
    link->timestamp = atol(ts_marker + 7);
    
    return GM_OK;
}

// List links
int gm_link_list(gm_link_set_t** set, const char* filter_source, const char* filter_target) {
    if (!set) {
        gm_set_error("Invalid argument");
        return GM_ERR_INVALID_ARG;
    }
    
    *set = gm_link_set_new();
    if (!*set) {
        return GM_ERR_MEMORY;
    }
    
    // Get links directory
    char cwd[GM_MAX_PATH];
    int ret = get_cwd(cwd, sizeof(cwd));
    if (ret != GM_OK) {
        gm_link_set_free(*set);
        return ret;
    }
    
    char links_dir[GM_MAX_PATH];
    if (gm_path_join(links_dir, sizeof(links_dir), cwd, ".gitmind/links") != GM_OK) {
        gm_link_set_free(*set);
        *set = NULL;
        return GM_ERR_PATH_TOO_LONG;
    }
    
    DIR* dir = opendir(links_dir);
    if (!dir) {
        // No links directory means no links
        return GM_OK;
    }
    
    struct dirent* entry;
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (entry->d_name[0] == '.') continue;
        
        // Check .link extension
        char* ext = strrchr(entry->d_name, '.');
        if (!ext || strcmp(ext, ".link") != 0) continue;
        
        // Build full path
        char filepath[GM_MAX_PATH];
        if (gm_path_join(filepath, sizeof(filepath), links_dir, entry->d_name) != GM_OK) {
            continue;  // Skip files with too-long paths
        }
        
        // Parse link
        gm_link_t link;
        if (parse_link_file(filepath, &link) != GM_OK) continue;
        
        // Apply filters
        if (filter_source && strcmp(link.source, filter_source) != 0) continue;
        if (filter_target && strcmp(link.target, filter_target) != 0) continue;
        
        // Add to set
        gm_link_set_add(*set, &link);
    }
    
    closedir(dir);
    return GM_OK;
}

// Remove link
int gm_link_unlink(const char* source, const char* target) {
    if (!source || !target) {
        gm_set_error("Invalid arguments");
        return GM_ERR_INVALID_ARG;
    }
    
    // List all links
    gm_link_set_t* set;
    int ret = gm_link_list(&set, source, target);
    if (ret != GM_OK) return ret;
    
    if (set->count == 0) {
        gm_link_set_free(set);
        gm_set_error("Link not found");
        return GM_ERR_NOT_FOUND;
    }
    
    // Remove each matching link
    char cwd[GM_MAX_PATH];
    ret = get_cwd(cwd, sizeof(cwd));
    if (ret != GM_OK) {
        gm_link_set_free(set);
        return ret;
    }
    
    for (size_t i = 0; i < set->count; i++) {
        gm_link_t* link = &set->links[i];
        
        // Build content to get SHA
        char content[8192];
        build_link_content(link, content, sizeof(content));
        
        char sha[GM_SHA1_STRING_SIZE];
        gm_sha1_string(content, sha);
        
        // Remove file
        char links_dir[GM_MAX_PATH];
        char filename[GM_MAX_PATH];
        char link_name[GM_MAX_PATH];
        
        // Build path safely
        if (gm_path_join(links_dir, sizeof(links_dir), cwd, ".gitmind/links") != GM_OK) {
            continue;  // Skip if path too long
        }
        
        snprintf(link_name, sizeof(link_name), "%s.link", sha);
        if (gm_path_join(filename, sizeof(filename), links_dir, link_name) != GM_OK) {
            continue;  // Skip if path too long
        }
        
        // Remove file and stage deletion
        unlink(filename);
        char cmd[GM_MAX_PATH * 2];
        snprintf(cmd, sizeof(cmd), "git add %s 2>/dev/null", filename);
        if (system(cmd) != 0) {
            // Ignore failure - git add is optional
        }
    }
    
    gm_link_set_free(set);
    return GM_OK;
}
