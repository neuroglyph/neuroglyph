// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
#include "errors.h"
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

// Check if file exists
static int file_exists(const char* path) {
    struct stat st;
    return (stat(path, &st) == 0 && S_ISREG(st.st_mode));
}

// Check link integrity
int gm_link_check(int fix, int* broken_count) {
    if (!broken_count) {
        gm_set_error("Invalid argument: broken_count is NULL");
        return GM_ERR_INVALID_ARG;
    }
    
    *broken_count = 0;
    
    // List all links
    gm_link_set_t* set;
    int ret = gm_link_list(&set, NULL, NULL);
    if (ret != GM_OK) return ret;
    
    // Track broken links
    int* broken_indices = NULL;
    if (fix) {
        broken_indices = calloc(set->count, sizeof(int));
        if (!broken_indices) {
            gm_link_set_free(set);
            gm_set_error("Memory allocation failed");
            return GM_ERR_MEMORY;
        }
    }
    
    // Check each link
    for (size_t i = 0; i < set->count; i++) {
        gm_link_t* link = &set->links[i];
        int source_exists = file_exists(link->source);
        int target_exists = file_exists(link->target);
        
        if (!source_exists || !target_exists) {
            (*broken_count)++;
            if (broken_indices) {
                broken_indices[i] = 1;
            }
            
            if (!fix) {
                fprintf(stderr, "Broken link: %s -> %s", link->source, link->target);
                if (!source_exists) fprintf(stderr, " (source missing)");
                if (!target_exists) fprintf(stderr, " (target missing)");
                fprintf(stderr, "\n");
            }
        }
    }
    
    // Fix broken links if requested
    if (fix && *broken_count > 0) {
        for (size_t i = 0; i < set->count; i++) {
            if (broken_indices[i]) {
                gm_link_t* link = &set->links[i];
                fprintf(stderr, "Removing broken link: %s -> %s\n", 
                    link->source, link->target);
                gm_link_unlink(link->source, link->target);
            }
        }
    }
    
    if (broken_indices) free(broken_indices);
    gm_link_set_free(set);
    
    return GM_OK;
}