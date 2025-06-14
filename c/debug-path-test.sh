#!/bin/bash
# Quick debug test
cd /test
echo "Testing simple path validation..."
cat > test_path.c << 'EOF'
#include <stdio.h>
#include "gitmind.h"

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <path>\n", argv[0]);
        return 1;
    }
    
    int result = gm_validate_link_path(argv[1]);
    printf("Path '%s' validation result: %d\n", argv[1], result);
    return 0;
}
EOF

gcc -o test_path test_path.c src/path.c src/gitmind.c -I./include -Wall -Wextra
./test_path "file.txt"
./test_path "src/main.c"
./test_path "../bad.txt"
./test_path "/etc/passwd"