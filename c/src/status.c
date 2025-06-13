// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
#include "errors.h"
#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/stat.h>

// Show gitmind status
int gm_status(void) {
    // Check if initialized
    char cwd[GM_MAX_PATH];
    if (!getcwd(cwd, sizeof(cwd))) {
        gm_set_error("Failed to get current directory");
        return GM_ERR_IO;
    }
    
    char links_dir[GM_MAX_PATH];
    if (gm_path_join(links_dir, sizeof(links_dir), cwd, GM_LINKS_DIR) != GM_OK) {
        return GM_ERR_PATH_TOO_LONG;
    }
    
    struct stat st;
    if (stat(links_dir, &st) != 0) {
        printf("GitMind: Not initialized (run 'gitmind init')\n");
        return GM_OK;
    }
    
    // List all links
    gm_link_set_t* set;
    int ret = gm_link_list(&set, NULL, NULL);
    if (ret != GM_OK) return ret;
    
    printf("GitMind Status\n");
    printf("==============\n");
    printf("Repository: %s\n", cwd);
    printf("Total links: %zu\n", set->count);
    
    if (set->count > 0) {
        // Count by type
        printf("\nLinks by type:\n");
        
        // Simple type counting (could be optimized with hash table)
        for (size_t i = 0; i < set->count; i++) {
            const char* type = set->links[i].type;
            int count = 1;
            
            // Skip if we've already counted this type
            int already_counted = 0;
            for (size_t j = 0; j < i; j++) {
                if (strcmp(set->links[j].type, type) == 0) {
                    already_counted = 1;
                    break;
                }
            }
            if (already_counted) continue;
            
            // Count occurrences
            for (size_t j = i + 1; j < set->count; j++) {
                if (strcmp(set->links[j].type, type) == 0) {
                    count++;
                }
            }
            
            printf("  %s: %d\n", type, count);
        }
        
        // Show last 5 links
        printf("\nRecent links:\n");
        size_t start = set->count > 5 ? set->count - 5 : 0;
        for (size_t i = start; i < set->count; i++) {
            gm_link_t* link = &set->links[i];
            
            // Format timestamp
            char time_buf[64];
            struct tm* tm_info = localtime(&link->timestamp);
            strftime(time_buf, sizeof(time_buf), "%Y-%m-%d %H:%M:%S", tm_info);
            
            printf("  [%s] %s -> %s (%s)\n", 
                time_buf, link->source, link->target, link->type);
        }
    }
    
    // Check for broken links
    int broken_count = 0;
    gm_link_check(0, &broken_count);
    if (broken_count > 0) {
        printf("\n⚠️  %d broken link%s found (run 'gitmind check --fix' to remove)\n", 
            broken_count, broken_count == 1 ? "" : "s");
    }
    
    gm_link_set_free(set);
    return GM_OK;
}