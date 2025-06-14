#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Test with multiple compilers in their strictest modes

set -euo pipefail

echo "🔧 Multi-Compiler Strict Build Test"
echo "==================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Base flags - maximum strictness
BASE_FLAGS="-std=c99 -Iinclude -O3"
STRICT_FLAGS="-Wall -Wextra -Werror -pedantic -pedantic-errors"

# Additional strict flags
EXTRA_STRICT="-Wcast-align -Wcast-qual -Wconversion -Wformat=2 -Wformat-security"
EXTRA_STRICT="$EXTRA_STRICT -Wmissing-declarations -Wmissing-prototypes -Wnested-externs"
EXTRA_STRICT="$EXTRA_STRICT -Wpointer-arith -Wredundant-decls -Wshadow -Wstrict-prototypes"
EXTRA_STRICT="$EXTRA_STRICT -Wundef -Wunreachable-code -Wwrite-strings"

# Source files
SOURCES="src/main.c src/git-mind.c src/link.c src/sha1.c src/path.c src/check.c src/status.c src/traverse.c"

# Test with each compiler
test_compiler() {
    local compiler="$1"
    local name="$2"
    local extra_flags="${3:-}"
    
    echo -e "\n${YELLOW}Testing with $name...${NC}"
    
    if ! command -v "$compiler" >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  $name not installed, skipping${NC}"
        return 0
    fi
    
    echo "Compiler version:"
    $compiler --version 2>&1 | head -1 || true
    
    # Clean first
    rm -f git-mind *.o src/*.o
    
    # Try to compile
    if $compiler $BASE_FLAGS $STRICT_FLAGS $EXTRA_STRICT $extra_flags -o git-mind $SOURCES 2>&1; then
        echo -e "${GREEN}✅ $name: Build successful${NC}"
        return 0
    else
        echo -e "${RED}❌ $name: Build failed${NC}"
        return 1
    fi
}

# Run tests
FAILED=0

# GCC with maximum warnings
if ! test_compiler "gcc" "GCC" "-Wformat-truncation=2 -Wformat-overflow=2 -Wstringop-truncation"; then
    ((FAILED++))
fi

# Clang with static analyzer (but not EVERYTHING - that's too pedantic)
if ! test_compiler "clang" "Clang" "-Wno-declaration-after-statement"; then
    ((FAILED++))
fi

# TinyCC for strict C99 compliance  
if ! test_compiler "tcc" "TinyCC" ""; then
    ((FAILED++))
fi

# Intel compiler if available (very strict)
if ! test_compiler "icc" "Intel C Compiler" "-w3 -Wremarks"; then
    ((FAILED++))
fi

# Summary
echo -e "\n${YELLOW}=== Summary ===${NC}"
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All compiler tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILED compiler(s) failed${NC}"
    exit 1
fi