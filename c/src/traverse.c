// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "gitmind.h"

// Simple set implementation for visited nodes
typedef struct {
    char** paths;
    size_t count;
    size_t capacity;
} gm_path_set_t;

// Queue node for BFS
typedef struct gm_queue_node {
    gm_traverse_node_t data;
    struct gm_queue_node* next;
} gm_queue_node_t;

// Queue for BFS traversal
typedef struct {
    gm_queue_node_t* head;
    gm_queue_node_t* tail;
} gm_queue_t;

// Create new path set
static gm_path_set_t* path_set_new(void) {
    gm_path_set_t* set = calloc(1, sizeof(gm_path_set_t));
    if (!set) return NULL;
    
    set->capacity = 16;
    set->paths = calloc(set->capacity, sizeof(char*));
    if (!set->paths) {
        free(set);
        return NULL;
    }
    
    return set;
}

// Free path set
static void path_set_free(gm_path_set_t* set) {
    if (!set) return;
    
    for (size_t i = 0; i < set->count; i++) {
        free(set->paths[i]);
    }
    free(set->paths);
    free(set);
}

// Check if path is in set
static bool path_set_contains(gm_path_set_t* set, const char* path) {
    for (size_t i = 0; i < set->count; i++) {
        if (strcmp(set->paths[i], path) == 0) {
            return true;
        }
    }
    return false;
}

// Helper to duplicate string (strdup not available in C99)
static char* duplicate_string(const char* str) {
    size_t len = strlen(str) + 1;
    char* copy = malloc(len);
    if (copy) {
        memcpy(copy, str, len);
    }
    return copy;
}

// Add path to set
static int path_set_add(gm_path_set_t* set, const char* path) {
    if (path_set_contains(set, path)) {
        return GM_OK;
    }
    
    if (set->count >= set->capacity) {
        size_t new_capacity = set->capacity * 2;
        char** new_paths = realloc(set->paths, new_capacity * sizeof(char*));
        if (!new_paths) {
            return GM_ERR_MEMORY;
        }
        set->paths = new_paths;
        set->capacity = new_capacity;
    }
    
    set->paths[set->count] = duplicate_string(path);
    if (!set->paths[set->count]) {
        return GM_ERR_MEMORY;
    }
    
    set->count++;
    return GM_OK;
}

// Create new queue
static gm_queue_t* queue_new(void) {
    return calloc(1, sizeof(gm_queue_t));
}

// Free queue
static void queue_free(gm_queue_t* queue) {
    if (!queue) return;
    
    gm_queue_node_t* node = queue->head;
    while (node) {
        gm_queue_node_t* next = node->next;
        free(node);
        node = next;
    }
    free(queue);
}

// Check if queue is empty
static bool queue_empty(gm_queue_t* queue) {
    return queue->head == NULL;
}

// Add node to queue
static int queue_push(gm_queue_t* queue, const gm_traverse_node_t* node) {
    gm_queue_node_t* new_node = calloc(1, sizeof(gm_queue_node_t));
    if (!new_node) {
        return GM_ERR_MEMORY;
    }
    
    new_node->data = *node;
    
    if (queue->tail) {
        queue->tail->next = new_node;
    } else {
        queue->head = new_node;
    }
    queue->tail = new_node;
    
    return GM_OK;
}

// Remove and return node from queue
static int queue_pop(gm_queue_t* queue, gm_traverse_node_t* out_node) {
    if (queue_empty(queue)) {
        return GM_ERR_NOT_FOUND;
    }
    
    gm_queue_node_t* node = queue->head;
    *out_node = node->data;
    
    queue->head = node->next;
    if (!queue->head) {
        queue->tail = NULL;
    }
    
    free(node);
    return GM_OK;
}

// Create new traverse result
gm_traverse_result_t* gm_traverse_result_new(void) {
    gm_traverse_result_t* result = calloc(1, sizeof(gm_traverse_result_t));
    if (!result) return NULL;
    
    result->capacity = 16;
    result->nodes = calloc(result->capacity, sizeof(gm_traverse_node_t));
    if (!result->nodes) {
        free(result);
        return NULL;
    }
    
    return result;
}

// Free traverse result
void gm_traverse_result_free(gm_traverse_result_t* result) {
    if (!result) return;
    free(result->nodes);
    free(result);
}

// Add node to traverse result
int gm_traverse_result_add(gm_traverse_result_t* result, const gm_traverse_node_t* node) {
    if (result->count >= result->capacity) {
        size_t new_capacity = result->capacity * 2;
        gm_traverse_node_t* new_nodes = realloc(result->nodes, 
                                                 new_capacity * sizeof(gm_traverse_node_t));
        if (!new_nodes) {
            return GM_ERR_MEMORY;
        }
        result->nodes = new_nodes;
        result->capacity = new_capacity;
    }
    
    result->nodes[result->count] = *node;
    result->count++;
    
    // Update counts
    if (node->depth == 1) {
        result->direct_count++;
    }
    result->total_count++;
    
    return GM_OK;
}

// Print tree format
void gm_traverse_print_tree(const gm_traverse_result_t* result, const char* start_node) {
    if (!result || !start_node) return;
    
    printf("%s (%d direct, %d total within depth)\n", 
           start_node, result->direct_count, result->total_count);
    
    if (result->count == 0) return;
    
    // Print nodes with tree formatting
    for (size_t i = 0; i < result->count; i++) {
        // Print indentation
        for (int j = 0; j < result->nodes[i].depth - 1; j++) {
            printf("|   ");
        }
        
        // Check if this is the last node at this depth
        bool is_last = true;
        for (size_t j = i + 1; j < result->count; j++) {
            if (result->nodes[j].depth == result->nodes[i].depth) {
                is_last = false;
                break;
            }
        }
        
        // Skip link type lookup for now - just use REFERENCES
        const char* link_type = "REFERENCES";
        
        // Print the node
        printf("%s %s [%s]\n", 
               is_last ? "\\->" : "+->",
               result->nodes[i].path,
               link_type);
    }
}

// Print list format
void gm_traverse_print_list(const gm_traverse_result_t* result, const char* start_node) {
    printf("%s (%d direct, %d total)\n", 
           start_node, result->direct_count, result->total_count);
    
    for (size_t i = 0; i < result->count; i++) {
        printf("  %s (depth: %d)\n", result->nodes[i].path, result->nodes[i].depth);
    }
}

// Cleanup helper - frees all allocated resources
static void cleanup_traverse(gm_path_set_t* visited, gm_queue_t* queue, 
                           gm_traverse_result_t** result) {
    path_set_free(visited);
    queue_free(queue);
    if (result && *result) {
        gm_traverse_result_free(*result);
        *result = NULL;
    }
}

// Main traverse function
int gm_traverse(const char* start_node, int depth, gm_format_t format, gm_traverse_result_t** result) {
    (void)format; // Currently unused, format handled by caller
    
    // Initialize these to NULL for cleanup
    gm_path_set_t* visited = NULL;
    gm_queue_t* queue = NULL;
    int ret = GM_OK;
    
    // Validate arguments
    if (!start_node || !result) {
        gm_set_error("Invalid arguments");
        return GM_ERR_INVALID_ARG;
    }
    
    *result = NULL;  // Initialize output
    
    // Check depth limits
    if (depth <= 0) {
        depth = GM_DEFAULT_DEPTH;
    }
    if (depth > GM_MAX_DEPTH) {
        gm_set_error("Depth exceeds maximum depth of %d", GM_MAX_DEPTH);
        return GM_ERR_INVALID_ARG;
    }
    
    // Check if start node exists
    char normalized_start[GM_MAX_PATH];
    if (gm_normalize_path(start_node, normalized_start) != GM_OK) {
        gm_set_error("Invalid path: %s", start_node);
        return GM_ERR_INVALID_ARG;
    }
    
    // Verify file exists
    FILE* f = fopen(normalized_start, "r");
    if (!f) {
        gm_set_error("File not found: %s", normalized_start);
        return GM_ERR_NOT_FOUND;
    }
    fclose(f);
    
    // Initialize result
    *result = gm_traverse_result_new();
    if (!*result) {
        return GM_ERR_MEMORY;
    }
    
    // Initialize visited set
    visited = path_set_new();
    if (!visited) {
        ret = GM_ERR_MEMORY;
        goto cleanup;
    }
    
    // Initialize queue
    queue = queue_new();
    if (!queue) {
        ret = GM_ERR_MEMORY;
        goto cleanup;
    }
    
    // Start BFS traversal
    gm_traverse_node_t start = {0};
    start.depth = 0;
    snprintf(start.path, GM_MAX_PATH, "%s", normalized_start);
    
    if (path_set_add(visited, normalized_start) != GM_OK ||
        queue_push(queue, &start) != GM_OK) {
        ret = GM_ERR_MEMORY;
        goto cleanup;
    }
    
    // BFS loop
    while (!queue_empty(queue)) {
        gm_traverse_node_t current;
        if (queue_pop(queue, &current) != GM_OK) {
            break;
        }
        
        // Skip root node in results
        if (current.depth > 0) {
            if (gm_traverse_result_add(*result, &current) != GM_OK) {
                ret = GM_ERR_MEMORY;
                goto cleanup;
            }
        }
        
        // Check depth limit
        if (current.depth >= depth) {
            continue;
        }
        
        // Get all links from current node
        gm_link_set_t* links = NULL;
        if (gm_link_list(&links, current.path, NULL) != GM_OK || !links) {
            continue;  // No links from this node
        }
        
        // Process each neighbor
        for (size_t i = 0; i < links->count; i++) {
            const char* neighbor = links->links[i].target;
            
            // Skip if already visited
            if (path_set_contains(visited, neighbor)) {
                continue;
            }
            
            // Add to visited set
            if (path_set_add(visited, neighbor) != GM_OK) {
                gm_link_set_free(links);
                ret = GM_ERR_MEMORY;
                goto cleanup;
            }
            
            // Create neighbor node
            gm_traverse_node_t neighbor_node = {0};
            neighbor_node.depth = current.depth + 1;
            snprintf(neighbor_node.path, GM_MAX_PATH, "%s", neighbor);
            snprintf(neighbor_node.parent, GM_MAX_PATH, "%s", current.path);
            
            // Add to queue
            if (queue_push(queue, &neighbor_node) != GM_OK) {
                gm_link_set_free(links);
                ret = GM_ERR_MEMORY;
                goto cleanup;
            }
        }
        
        gm_link_set_free(links);
    }
    
    // Success path
    path_set_free(visited);
    queue_free(queue);
    return GM_OK;

cleanup:
    // Error path - clean up everything
    cleanup_traverse(visited, queue, result);
    return ret;
}
