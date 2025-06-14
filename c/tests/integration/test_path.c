#include <stdio.h>
#include <string.h>
#include "gitmind.h"

// Test wrapper
int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <path>\n", argv[0]);
        return 1;
    }
    
    int result = gm_validate_link_path(argv[1]);
    printf("%d\n", result);
    return 0;
}
