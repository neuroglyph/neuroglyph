# GitMind C Implementation

Pure C. No BS. 65KB binary. Zero dependencies.

## Build

```bash
make
```

That's it. You get a `gitmind` binary.

## Test

```bash
./docker-test.sh  # Safe, isolated tests
```

## Philosophy

- **Integration tests first** - Bash scripts test real behavior
- **Unit tests later** - When we need to hammer edge cases
- **Fuzz when stable** - AFL/libFuzzer for finding crashes
- **Valgrind always** - Memory correctness matters

## Future Testing Strategy

### Phase 1: Integration (DONE ✓)
- Bash tests covering all commands
- Real git repos, real operations
- Runs in Docker for safety

### Phase 2: Edge Cases (TODO)
```c
// test_edge_cases.c
void test_path_with_spaces() {
    assert(gm_link_create("my file.md", "your file.md", "REFS") == GM_OK);
}

void test_unicode_paths() {
    assert(gm_link_create("文档.md", "ドキュメント.md", "REFS") == GM_OK);
}

void test_massive_link_count() {
    for (int i = 0; i < 100000; i++) {
        // Hammer it
    }
}
```

### Phase 3: Fuzz Testing
```bash
# AFL fuzzing
afl-fuzz -i seeds -o findings ./gitmind_fuzz @@

# libFuzzer
clang -fsanitize=fuzzer,address gitmind_fuzz.c -o fuzz
./fuzz corpus/
```

### Phase 4: Stress Testing
- 1 million links
- Concurrent operations
- Corrupted git repos
- Full disk scenarios

## Memory Testing

```bash
# Valgrind memcheck
valgrind --leak-check=full ./gitmind list

# AddressSanitizer (built-in)
make debug
./gitmind list  # ASAN will catch issues

# Static analysis
scan-build make
```

## Why This Approach

1. **Start simple** - If bash can test it, bash tests it
2. **Add complexity only when needed** - YAGNI
3. **Real usage first** - Integration > Unit
4. **Safety always** - Docker isolation, sanitizers

The goal: Ship fast, stay correct, add sophistication only when earned.