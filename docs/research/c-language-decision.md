# C Language Decision for GitMind Core

## Why C is Actually Perfect for GitMind

### The Game Dev Perspective
You know what? You're right. As a game dev, you understand:
- Memory is real, not abstract
- Performance matters at every level
- Simple code is debuggable code
- Direct control beats abstraction layers
- If it compiles, it probably works

### What GitMind Needs (Revisited for C)

1. **File I/O** - C's bread and butter
2. **String manipulation** - We'll manage our own buffers
3. **SHA-1 hashing** - OpenSSL or tiny implementation
4. **Process spawning** - fork/exec for git commands
5. **Cross-platform** - #ifdef and we're good

### C vs The World

| Feature | C | Go | Rust | Why C Wins |
|---------|---|----|----|------------|
| Binary size | ~50KB | 10MB | 5MB | Fits in L1 cache |
| Dependencies | libc | Go runtime | Rust runtime | Already on system |
| Startup time | 0.001s | 0.01s | 0.01s | Instant |
| Memory usage | What you malloc | 10MB minimum | 5MB minimum | Only what we need |
| Complexity | Simple | Medium | High | No hidden magic |
| Debugging | gdb/valgrind | dlv | rust-gdb | Battle-tested tools |

### The GitMind C Architecture

```c
// gitmind.h
#ifndef GITMIND_H
#define GITMIND_H

#include <time.h>

#define MAX_PATH 4096
#define MAX_LINK_TYPE 64
#define SHA1_SIZE 20
#define SHA1_STRING_SIZE 41

typedef struct {
    char type[MAX_LINK_TYPE];
    char source[MAX_PATH];
    char target[MAX_PATH];
    time_t timestamp;
} gm_link_t;

typedef struct {
    unsigned char bytes[SHA1_SIZE];
    char string[SHA1_STRING_SIZE];
} gm_sha1_t;

// Core operations
int gm_init(const char* repo_path);
int gm_link_create(const char* source, const char* target, const char* type);
int gm_link_list(gm_link_t** links, int* count);
int gm_link_unlink(const char* source, const char* target);
int gm_link_check(int fix);

// Utilities
void gm_sha1_compute(const char* content, gm_sha1_t* sha);
int gm_git_add(const char* filepath);
int gm_git_commit(const char* message);

#endif
```

### Why This Will Be Fast AF

1. **Zero-copy operations** - mmap the link files
2. **Stack allocation** - Most operations need no malloc
3. **Direct syscalls** - No runtime overhead
4. **Inline everything** - Let the compiler go wild
5. **Cache-friendly** - Struct packing for speed

### Implementation Plan

```c
// Day 1: Core structures and init
typedef struct {
    int fd;
    char* base;
    size_t size;
} mmap_file_t;

// Day 2: Link operations
int create_link(const char* source, const char* target, const char* type) {
    char buffer[8192];
    int len = snprintf(buffer, sizeof(buffer), 
        "%s: %s -> %s  # ts:%ld\n", 
        type, source, target, time(NULL));
    
    // SHA1 hash for filename
    unsigned char hash[20];
    SHA1((unsigned char*)buffer, len, hash);
    
    // Write to .gitmind/links/
    char filename[128];
    sha1_to_hex(hash, filename);
    
    return write_file(filename, buffer, len);
}

// Day 3: Git integration (just shell out, it's fine)
int git_add(const char* file) {
    return system_fmt("git add %s", file);
}
```

### Memory Management Strategy

```c
// Static buffers for most operations
static char g_path_buffer[MAX_PATH];
static char g_link_buffer[8192];

// Pool allocator for dynamic lists
typedef struct pool_block {
    struct pool_block* next;
    size_t used;
    char data[];
} pool_block_t;

// Only malloc when absolutely necessary
gm_link_t* links = pool_alloc(&link_pool, sizeof(gm_link_t) * count);
```

### Cross-Platform Without the BS

```c
#ifdef _WIN32
    #define PATH_SEP "\\"
    #include <windows.h>
#else
    #define PATH_SEP "/"
    #include <unistd.h>
#endif

// That's it. No 50MB of platform abstraction.
```

### Testing Strategy

```c
// Unit tests in pure C
void test_create_link() {
    assert(gm_init("/tmp/test_repo") == 0);
    assert(gm_link_create("a.md", "b.md", "REFS") == 0);
    
    gm_link_t* links;
    int count;
    assert(gm_link_list(&links, &count) == 0);
    assert(count == 1);
    assert(strcmp(links[0].source, "a.md") == 0);
}

// Valgrind for memory leaks
// AFL for fuzzing
// Simple and bulletproof
```

### Distribution

- Single binary < 100KB
- Static link everything optional
- Works on potato computers
- No install wizard BS
- Just download and run

### The Power Move

While everyone else is arguing about async runtimes and borrow checkers, we ship a tool that:
- Starts instantly
- Uses 500KB of RAM
- Runs on everything
- Has zero dependencies
- Actually works

### Performance Targets

- `gitmind init`: < 1ms
- `gitmind link`: < 5ms
- `gitmind list` (1000 links): < 10ms
- Memory usage: < 1MB for any operation
- Binary size: < 100KB stripped

### Let's Build This

```bash
# Day 1
cc -O3 -Wall -c gitmind.c
cc -O3 -Wall -c sha1.c
cc -O3 -Wall -c main.c
cc -o gitmind *.o -lcrypto

# Day 7
./gitmind init
./gitmind link README.md INSTALL.md
# Done. Ship it.
```

## Conclusion

C gives us:
- Absolute performance
- Tiny binaries
- Zero bullshit
- Complete control
- Runs everywhere

You + Me + C = GitMind that starts before your finger leaves the Enter key.

Ready to malloc this thing into existence?