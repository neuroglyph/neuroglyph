# Phase 1 Security & Memory Fixes Tracking

**Status**: Ready to implement  
**Priority**: Critical - must fix before any new features  
**Created**: June 14, 2025

## 🔥 Must-Fix Bugs Checklist

### 1. Path Traversal Security
- [ ] Write proper path parser that catches all `..` variants
  - `../`, `..\\`, `foo/../bar`, `./../../etc`
  - Unicode variants, URL encoding
- [ ] Add comprehensive test cases for path validation
- [ ] Replace `strstr("..")` in `path.c:55`

### 2. Memory Leaks
- [ ] **traverse.c**: Add `path_set_free()` on all early returns
  - Line ~150-250: Multiple return paths without cleanup
- [ ] **check.c**: Fix leak when `broken_indices` alloc fails
  - Line ~36: Need to free `set` before return
- [ ] Audit all `malloc`/`calloc` paths with:
  ```bash
  valgrind --leak-check=full --show-leak-kinds=all ./gitmind
  ```

### 3. Error Code Consistency  
- [ ] Global search/replace: `return 0;` → `return GM_OK;`
- [ ] Affected files:
  - `gitmind.c`: `ensure_dir()` returns raw 0
  - All helper functions should return `gm_error_t`

### 4. Thread-Local Portability
- [ ] Add to `gitmind.c`:
  ```c
  #if __STDC_VERSION__ >= 201112L
    #define THREAD_LOCAL _Thread_local
  #elif defined(__GNUC__) || defined(__clang__)
    #define THREAD_LOCAL __thread
  #else
    #define THREAD_LOCAL  // fallback: no TLS
  #endif
  
  static THREAD_LOCAL char gm_err_buf[GM_ERROR_BUFFER_SIZE];
  ```

### 5. O(n) Type Counting
- [ ] Implement simple fixed-size hash table (~30 lines)
- [ ] Replace O(n²) loop in `status.c:46-69`
- [ ] Consider: Open addressing, linear probing, fixed buckets

## Implementation Order

1. **Path security first** - it's a real vulnerability
2. **Memory leaks second** - ruins our valgrind hygiene  
3. **Thread-local macro** - quick fix, improves portability
4. **Error consistency** - mechanical fix, improves debugging
5. **Hash table last** - performance, not correctness

## Verification

After each fix:
```bash
# Security check
./test-path-validation.sh  # Write this!

# Memory check
valgrind --leak-check=full --error-exitcode=1 make test

# Build portability
CC=clang make clean all
CC=gcc make clean all
```

## Notes

- Keep the "1999 dagger" philosophy - no external deps
- Each fix should be a single, focused commit
- Update tests as we go
- Document any new constraints in CLAUDE.md

---

Phase 2 can wait. Let's fix these bugs and ship something solid.