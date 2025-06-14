#!/bin/bash
# Test minimal compilation
cd /test

cat > test_minimal.c << 'EOF'
#include <stdio.h>
#include "gitmind.h"

int main() {
    int result = gm_validate_link_path("test.txt");
    printf("Result: %d\n", result);
    return 0;
}
EOF

echo "Trying to compile with all source files..."
gcc -o test_minimal test_minimal.c src/*.c -I./include -Wall -Wextra 2>&1

if [ $? -eq 0 ]; then
    echo "Compilation successful!"
    ./test_minimal
else
    echo "Compilation failed"
fi