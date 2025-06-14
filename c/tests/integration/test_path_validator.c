#include <stdio.h>
#include <string.h>
#include "gitmind.h"

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <path>\n", argv[0]);
        return 1;
    }
    
    int result = gm_validate_link_path(argv[1]);
    if (result == GM_OK) {
        printf("VALID\n");
        return 0;
    } else {
        printf("INVALID: %s\n", gm_last_error());
        return 1;
    }
}
