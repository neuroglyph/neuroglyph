// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

#ifndef GITMIND_ERRORS_H
#define GITMIND_ERRORS_H

// Error message constants - no typos, easy i18n, consistent messaging
#define ERR_MSG_NOT_REPO "Not a git repository: %s"
#define ERR_MSG_NOT_FOUND "Not found: %s"
#define ERR_MSG_IO_ERROR "I/O error: %s"
#define ERR_MSG_GIT_FAILED "Git operation failed: %s"
#define ERR_MSG_MEMORY "Memory allocation failed"
#define ERR_MSG_INVALID_ARG "Invalid argument: %s"
#define ERR_MSG_PATH_TOO_LONG "Path too long: %s"
#define ERR_MSG_ALREADY_EXISTS "Already exists: %s"
#define ERR_MSG_EMPTY_PATH "Empty path"
#define ERR_MSG_ABSOLUTE_PATH "Absolute paths not allowed in links: %s"
#define ERR_MSG_PATH_TRAVERSAL "Path traversal not allowed: %s"
#define ERR_MSG_LINK_NOT_FOUND "Link not found"
#define ERR_MSG_DIR_CREATE_FAILED "Failed to create directory %s: %s"
#define ERR_MSG_FILE_CREATE_FAILED "Failed to create file: %s"
#define ERR_MSG_FILE_WRITE_FAILED "Failed to write file"
#define ERR_MSG_LINK_CONTENT_TOO_LONG "Link content too long"
#define ERR_MSG_CWD_FAILED "Failed to get current directory"
#define ERR_MSG_NULL_POINTER "NULL pointer provided for %s"

// CLI error messages
#define ERR_MSG_LINK_REQUIRES_ARGS "Error: link requires source and target arguments\n"
#define ERR_MSG_UNLINK_REQUIRES_ARGS "Error: unlink requires source and target arguments\n"
#define ERR_MSG_UNKNOWN_COMMAND "Error: Unknown command '%s'\n"

// Status messages
#define MSG_INIT_SUCCESS "Initialized gitmind in current repository\n"
#define MSG_LINK_CREATED "Created link: %s -> %s (%s)\n"
#define MSG_LINK_REMOVED "Removed link: %s -> %s\n"
#define MSG_NO_LINKS "No links found\n"
#define MSG_ALL_LINKS_VALID "All links are valid\n"
#define MSG_BROKEN_LINKS_FOUND "Found %d broken link%s\n"
#define MSG_BROKEN_LINKS_REMOVED "Removed %d broken link%s\n"
#define MSG_RUN_CHECK_FIX "Run 'gitmind check --fix' to remove them\n"
#define MSG_NOT_INITIALIZED "GitMind: Not initialized (run 'gitmind init')\n"

// Version info
#define MSG_VERSION_FORMAT "gitmind version %s\n"

#endif // GITMIND_ERRORS_H