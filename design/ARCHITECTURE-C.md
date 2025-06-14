# GitMind C Architecture

## Core Design Principles

1. **Zero-copy where possible** - mmap files, work in-place
2. **Stack-first allocation** - Malloc only when necessary
3. **Simple data structures** - Arrays > linked lists
4. **Direct syscalls** - No abstraction layers
5. **Fail fast** - Return error codes, let caller decide

## Architecture Overview

```
gitmind (binary ~50KB)
├── Core (.gitmind/links/)
│   ├── SHA1-based filenames
│   ├── Plain text format
│   └── Git integration
├── Commands
│   ├── init   - Create .gitmind/
│   ├── link   - Create link file
│   ├── list   - Read all links
│   ├── unlink - Remove link file
│   └── check  - Validate links
└── Utilities
    ├── SHA1 computation
    ├── Path normalization
    └── Git subprocess
```

## Data Structures

```c
// Fixed-size structures for stack allocation
typedef struct {
    char type[64];      // REFERENCES, IMPLEMENTS, etc.
    char source[4096];  // Source file path
    char target[4096];  // Target file path
    time_t timestamp;   // Unix timestamp
} gm_link_t;

// Link collection for batch operations
typedef struct {
    gm_link_t* links;   // Dynamic array
    size_t count;       // Number of links
    size_t capacity;    // Allocated capacity
} gm_link_set_t;

// Error handling
typedef enum {
    GM_OK = 0,
    GM_ERR_NOT_REPO = -1,
    GM_ERR_NOT_FOUND = -2,
    GM_ERR_IO = -3,
    GM_ERR_GIT = -4,
    GM_ERR_MEMORY = -5
} gm_error_t;
```

## Memory Management

### Strategy 1: Static Buffers
```c
// Thread-local storage for temporary operations
static __thread char g_path_buffer[8192];
static __thread char g_sha_buffer[41];
static __thread char g_cmd_buffer[4096];
```

### Strategy 2: Arena Allocator
```c
typedef struct {
    char* base;
    size_t size;
    size_t used;
} arena_t;

// Allocate from arena, free everything at once
void* arena_alloc(arena_t* arena, size_t size);
void arena_reset(arena_t* arena);
```

### Strategy 3: mmap for Large Data
```c
// Map entire .gitmind/links/ directory
typedef struct {
    void* base;
    size_t size;
    int fd;
} mmap_region_t;
```

## Core Operations

### 1. Init Command
```c
int gm_init(const char* repo_path) {
    // 1. Check if .git exists
    // 2. Create .gitmind/links/
    // 3. Add to .gitignore if needed
    // 4. Return GM_OK or error
}
```

### 2. Link Creation
```c
int gm_link_create(const char* source, const char* target, const char* type) {
    // 1. Normalize paths
    // 2. Build link content: "TYPE: source -> target  # ts:12345"
    // 3. Compute SHA1 of content
    // 4. Write to .gitmind/links/<sha>.link
    // 5. Git add the file
    // 6. Return GM_OK or error
}
```

### 3. Link Listing
```c
int gm_link_list(gm_link_set_t* set, const char* filter_source, const char* filter_target) {
    // 1. opendir(".gitmind/links")
    // 2. Read each .link file
    // 3. Parse into gm_link_t
    // 4. Apply filters if provided
    // 5. Add to set
    // 6. Return count or error
}
```

### 4. Link Removal
```c
int gm_link_unlink(const char* source, const char* target) {
    // 1. Find matching link file(s)
    // 2. Git rm the file(s)
    // 3. Return GM_OK or error
}
```

### 5. Link Checking
```c
int gm_link_check(int fix, int* broken_count) {
    // 1. List all links
    // 2. Check if source/target exist
    // 3. If fix, remove broken links
    // 4. Return count of broken links
}
```

## File Format

```
# .gitmind/links/a3f5c8d9e2b1.link
REFERENCES: docs/api.md -> src/server.c  # ts:1718921045
```

- One link per file
- Filename is SHA1 of content
- Human-readable format
- Git-trackable

## Git Integration

```c
// Simple subprocess execution
int git_run(const char* fmt, ...) {
    char cmd[4096];
    va_list args;
    va_start(args, fmt);
    vsnprintf(cmd, sizeof(cmd), fmt, args);
    va_end(args);
    
    return system(cmd);
}

// Usage
git_run("git add %s", filepath);
git_run("git commit -m 'link: %s -> %s'", source, target);
```

## Performance Characteristics

| Operation | Target Time | Method |
|-----------|------------|---------|
| init | < 1ms | mkdir + stat |
| link | < 5ms | write + git add |
| list (1000) | < 10ms | readdir + parse |
| unlink | < 5ms | unlink + git rm |
| check (1000) | < 20ms | stat batch |

## Build System

```makefile
# Minimal Makefile
CC = cc
CFLAGS = -O3 -Wall -Wextra -std=c99
LDFLAGS = -lcrypto

SRCS = main.c gitmind.c sha1.c git.c
OBJS = $(SRCS:.c=.o)

gitmind: $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)
	strip $@

clean:
	rm -f gitmind $(OBJS)

.PHONY: clean
```

## Testing Approach

```c
// Unit tests in test.c
void test_sha1() {
    char hash[41];
    gm_sha1_string("test", hash);
    assert(strcmp(hash, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3") == 0);
}

void test_link_parse() {
    gm_link_t link;
    assert(parse_link_file("REFS: a.md -> b.md  # ts:12345", &link) == 0);
    assert(strcmp(link.type, "REFS") == 0);
    assert(strcmp(link.source, "a.md") == 0);
    assert(link.timestamp == 12345);
}

// Integration tests with temp repos
void test_full_flow() {
    char tmpdir[] = "/tmp/gitmind_test_XXXXXX";
    mkdtemp(tmpdir);
    
    system_fmt("cd %s && git init", tmpdir);
    assert(gm_init(tmpdir) == 0);
    assert(gm_link_create("a.md", "b.md", "REFS") == 0);
    
    // Cleanup
    system_fmt("rm -rf %s", tmpdir);
}
```

## Platform Support

```c
#ifdef _WIN32
    #include <windows.h>
    #define PATH_MAX MAX_PATH
    #define mkdir(p,m) _mkdir(p)
#else
    #include <unistd.h>
    #include <dirent.h>
#endif
```

## Future Optimizations

1. **Batch git operations** - Queue adds, single commit
2. **Index file** - Binary format for faster lookups
3. **Memory-mapped links** - mmap entire directory
4. **SIMD SHA1** - Hardware acceleration
5. **Custom allocator** - Per-command arena

## Why This Architecture

- **Simple** - ~1000 lines of C total
- **Fast** - No overhead, direct operations
- **Portable** - POSIX + minimal Windows shims
- **Debuggable** - gdb, valgrind, printf
- **Extensible** - Clean C API for bindings

Ready to implement?