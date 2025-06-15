// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/stat.h>

// Fixed-size hash table for type counting
#define TYPE_HASH_SIZE 64

// Type entry for hash table
typedef struct type_entry {
    char type[GM_MAX_TYPE];
    int count;
    struct type_entry* next;
} type_entry_t;

// Simple hash function for type strings
static unsigned int type_hash(const char* str) {
    unsigned int h = 5381;
    int c;
    while ((c = *str++))
        h = ((h << 5) + h) + (unsigned int)c;
    return h % TYPE_HASH_SIZE;
}

// Free hash table
static void free_hash_table(type_entry_t* hash_table[TYPE_HASH_SIZE]) {
    for (int i = 0; i < TYPE_HASH_SIZE; i++) {
        type_entry_t* entry = hash_table[i];
        while (entry) {
            type_entry_t* next = entry->next;
            free(entry);
            entry = next;
        }
        hash_table[i] = NULL;
    }
}

// Print type counts from hash table
static void print_type_counts(type_entry_t* hash_table[TYPE_HASH_SIZE]) {
    for (int i = 0; i < TYPE_HASH_SIZE; i++) {
        type_entry_t* entry = hash_table[i];
        while (entry) {
            printf("  %s: %d\n", entry->type, entry->count);
            entry = entry->next;
        }
    }
}

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
    gm_link_set_t* set = NULL;
    int ret = gm_link_list(&set, NULL, NULL);
    if (ret != GM_OK) return ret;
    
    printf("GitMind Status\n");
    printf("==============\n");
    printf("Repository: %s\n", cwd);
    printf("Total links: %zu\n", set->count);
    
    if (set->count > 0) {
        // Count by type using simple hash table
        printf("\nLinks by type:\n");
        
        type_entry_t* hash_table[TYPE_HASH_SIZE] = {0};
        int allocation_failed = 0;
        
        // Count each type
        for (size_t i = 0; i < set->count && !allocation_failed; i++) {
            const char* type = set->links[i].type;
            unsigned int h = type_hash(type);
            
            // Look for existing entry
            type_entry_t* entry = hash_table[h];
            type_entry_t* prev = NULL;
            while (entry && strcmp(entry->type, type) != 0) {
                prev = entry;
                entry = entry->next;
            }
            
            if (entry) {
                // Found existing type
                entry->count++;
            } else {
                // New type
                type_entry_t* new_entry = malloc(sizeof(type_entry_t));
                if (!new_entry) {
                    // Allocation failed - clean up and continue
                    allocation_failed = 1;
                    free_hash_table(hash_table);
                    printf("  (Memory allocation failed - type counts unavailable)\n");
                } else {
                    snprintf(new_entry->type, GM_MAX_TYPE, "%s", type);
                    new_entry->count = 1;
                    new_entry->next = NULL;
                    
                    if (prev) {
                        prev->next = new_entry;
                    } else {
                        hash_table[h] = new_entry;
                    }
                }
            }
        }
        
        // Print and free hash table (if allocation succeeded)
        if (!allocation_failed) {
            print_type_counts(hash_table);
            free_hash_table(hash_table);
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
