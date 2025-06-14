# GitMind C Implementation

Pure C. No BS. 130KB binary. Zero dependencies.

## Build

```bash
make
```

That's it. You get a `git-mind` binary.

## Test

```bash
# Run all tests in Docker (safe, isolated)
docker compose run test

# Or run specific test suites
./tests/integration/test.sh        # Main test suite
./tests/integration/docker-test.sh # Full Docker test
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
afl-fuzz -i seeds -o findings ./git-mind_fuzz @@

# libFuzzer
clang -fsanitize=fuzzer,address git-mind_fuzz.c -o fuzz
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
valgrind --leak-check=full ./git-mind list

# AddressSanitizer (built-in)
make debug
./git-mind list  # ASAN will catch issues

# Static analysis
scan-build make
```

## Why This Approach

1. **Start simple** - If bash can test it, bash tests it
2. **Add complexity only when needed** - YAGNI
3. **Real usage first** - Integration > Unit
4. **Safety always** - Docker isolation, sanitizers

The goal: Ship fast, stay correct, add sophistication only when earned.