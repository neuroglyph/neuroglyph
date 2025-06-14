// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#include "gitmind.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h>

static void print_usage(const char* prog) {
    fprintf(stderr, "Usage: %s <command> [options]\n", prog);
    fprintf(stderr, "\nCommands:\n");
    fprintf(stderr, "  init                    Initialize gitmind in current repository\n");
    fprintf(stderr, "  link <source> <target>  Create a link between files\n");
    fprintf(stderr, "    --type TYPE           Link type (default: REFERENCES)\n");
    fprintf(stderr, "  list                    List all links\n");
    fprintf(stderr, "    --source FILE         Filter by source file\n");
    fprintf(stderr, "    --target FILE         Filter by target file\n");
    fprintf(stderr, "  traverse <file>         Traverse graph from file\n");
    fprintf(stderr, "    --depth N             Traversal depth (default: 1, max: 10)\n");
    fprintf(stderr, "    --format FORMAT       Output format: tree, list (default: tree)\n");
    fprintf(stderr, "  unlink <source> <target> Remove link between files\n");
    fprintf(stderr, "  check                   Check link integrity\n");
    fprintf(stderr, "    --fix                 Remove broken links\n");
    fprintf(stderr, "  status                  Show repository status\n");
    fprintf(stderr, "  version                 Show version\n");
}

static int cmd_init(int argc, char** argv) {
    (void)argc;
    (void)argv;
    
    int ret = gm_init(".");
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    printf("Initialized gitmind in current repository\n");
    return 0;
}

static int cmd_link(int argc, char** argv) {
    const char* type = "REFERENCES";
    
    // Parse options starting from argv[2] (after "gitmind link")
    int opt;
    static struct option long_options[] = {
        {"type", required_argument, 0, 't'},
        {0, 0, 0, 0}
    };
    
    optind = 2; // Start after "gitmind link"
    while ((opt = getopt_long(argc, argv, "t:", long_options, NULL)) != -1) {
        switch (opt) {
            case 't':
                type = optarg;
                break;
            default:
                print_usage(argv[0]);
                return 1;
        }
    }
    
    // Need exactly 2 arguments after options
    if (optind + 2 != argc) {
        fprintf(stderr, "Error: link requires source and target arguments\n");
        print_usage(argv[0]);
        return 1;
    }
    
    const char* source = argv[optind];
    const char* target = argv[optind + 1];
    
    int ret = gm_link_create(source, target, type);
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    printf("Created link: %s -> %s (%s)\n", source, target, type);
    return 0;
}

static int cmd_list(int argc, char** argv) {
    const char* filter_source = NULL;
    const char* filter_target = NULL;
    
    // Parse options
    int opt;
    static struct option long_options[] = {
        {"source", required_argument, 0, 's'},
        {"target", required_argument, 0, 't'},
        {0, 0, 0, 0}
    };
    
    optind = 2; // Start after "gitmind list"
    while ((opt = getopt_long(argc, argv, "s:t:", long_options, NULL)) != -1) {
        switch (opt) {
            case 's':
                filter_source = optarg;
                break;
            case 't':
                filter_target = optarg;
                break;
            default:
                print_usage(argv[0]);
                return 1;
        }
    }
    
    gm_link_set_t* set;
    int ret = gm_link_list(&set, filter_source, filter_target);
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    if (set->count == 0) {
        printf("No links found\n");
    } else {
        for (size_t i = 0; i < set->count; i++) {
            gm_link_t* link = &set->links[i];
            printf("%s: %s -> %s (ts:%ld)\n", 
                link->type, link->source, link->target, link->timestamp);
        }
    }
    
    gm_link_set_free(set);
    return 0;
}

static int cmd_unlink(int argc, char** argv) {
    if (argc != 4) {
        fprintf(stderr, "Error: unlink requires source and target arguments\n");
        print_usage(argv[0]);
        return 1;
    }
    
    const char* source = argv[2];
    const char* target = argv[3];
    
    int ret = gm_link_unlink(source, target);
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    printf("Removed link: %s -> %s\n", source, target);
    return 0;
}

static int cmd_check(int argc, char** argv) {
    int fix = 0;
    
    // Check for --fix flag
    for (int i = 2; i < argc; i++) {
        if (strcmp(argv[i], "--fix") == 0) {
            fix = 1;
            break;
        }
    }
    
    int broken_count = 0;
    int ret = gm_link_check(fix, &broken_count);
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    if (broken_count == 0) {
        printf("All links are valid\n");
    } else if (!fix) {
        printf("Found %d broken link%s\n", broken_count, broken_count == 1 ? "" : "s");
        printf("Run 'gitmind check --fix' to remove them\n");
    } else {
        printf("Removed %d broken link%s\n", broken_count, broken_count == 1 ? "" : "s");
    }
    
    return 0;
}

static int cmd_status(int argc, char** argv) {
    (void)argc;
    (void)argv;
    
    int ret = gm_status();
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    return 0;
}

static int cmd_version(int argc, char** argv) {
    (void)argc;
    (void)argv;
    
    printf("gitmind version %s\n", gm_version_string());
    return 0;
}

static int cmd_traverse(int argc, char** argv) {
    if (argc < 3) {
        fprintf(stderr, "Error: Missing file argument\n");
        fprintf(stderr, "Usage: gitmind traverse <file> [options]\n");
        return 1;
    }
    
    const char* file = argv[2];
    int depth = GM_DEFAULT_DEPTH;
    gm_format_t format = GM_FORMAT_TREE;
    
    // Parse options
    int opt;
    static struct option long_options[] = {
        {"depth", required_argument, 0, 'd'},
        {"format", required_argument, 0, 'f'},
        {0, 0, 0, 0}
    };
    
    optind = 3; // Start after "gitmind traverse <file>"
    while ((opt = getopt_long(argc, argv, "d:f:", long_options, NULL)) != -1) {
        switch (opt) {
        case 'd':
            depth = atoi(optarg);
            if (depth <= 0 || depth > GM_MAX_DEPTH) {
                fprintf(stderr, "Error: Depth must be between 1 and %d\n", GM_MAX_DEPTH);
                return 1;
            }
            break;
        case 'f':
            if (strcmp(optarg, "tree") == 0) {
                format = GM_FORMAT_TREE;
            } else if (strcmp(optarg, "list") == 0) {
                format = GM_FORMAT_LIST;
            } else {
                fprintf(stderr, "Error: Unknown format '%s' (use 'tree' or 'list')\n", optarg);
                return 1;
            }
            break;
        default:
            return 1;
        }
    }
    
    gm_traverse_result_t* result = NULL;
    int ret = gm_traverse(file, depth, format, &result);
    if (ret != GM_OK) {
        fprintf(stderr, "Error: %s\n", gm_last_error());
        return 1;
    }
    
    // Print results based on format
    if (format == GM_FORMAT_TREE) {
        gm_traverse_print_tree(result, file);
    } else {
        gm_traverse_print_list(result, file);
    }
    
    gm_traverse_result_free(result);
    return 0;
}

int main(int argc, char** argv) {
    if (argc < 2) {
        print_usage(argv[0]);
        return 1;
    }
    
    const char* cmd = argv[1];
    
    if (strcmp(cmd, "init") == 0) {
        return cmd_init(argc, argv);
    } else if (strcmp(cmd, "link") == 0) {
        return cmd_link(argc, argv);
    } else if (strcmp(cmd, "list") == 0) {
        return cmd_list(argc, argv);
    } else if (strcmp(cmd, "unlink") == 0) {
        return cmd_unlink(argc, argv);
    } else if (strcmp(cmd, "check") == 0) {
        return cmd_check(argc, argv);
    } else if (strcmp(cmd, "status") == 0) {
        return cmd_status(argc, argv);
    } else if (strcmp(cmd, "traverse") == 0) {
        return cmd_traverse(argc, argv);
    } else if (strcmp(cmd, "version") == 0) {
        return cmd_version(argc, argv);
    } else {
        fprintf(stderr, "Error: Unknown command '%s'\n", cmd);
        print_usage(argv[0]);
        return 1;
    }
}