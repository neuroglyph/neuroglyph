# SPDX-License-Identifier: Apache-2.0
# Simple Makefile for quick builds

CC ?= cc
CFLAGS ?= -O3 -Wall -Wextra -std=c99 -Iinclude
LDFLAGS ?=

# Platform detection
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    # Linux-specific flags (static linking possible)
    LDFLAGS += -static
endif
ifeq ($(UNAME_S),Darwin)
    # macOS doesn't support full static linking
    # Use -arch flags if provided
endif

# Windows/MinGW detection
ifeq ($(OS),Windows_NT)
    TARGET = gitmind.exe
else
    TARGET = gitmind
endif

SRCS = src/main.c src/gitmind.c src/link.c src/sha1.c src/path.c src/check.c src/status.c src/traverse.c
OBJS = $(SRCS:.c=.o)

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)
	strip $@

debug: CFLAGS = -g -O0 -Wall -Wextra -std=c99 -Iinclude -fsanitize=address,undefined
debug: LDFLAGS += -fsanitize=address,undefined
debug: $(TARGET)

clean:
	rm -f $(TARGET) $(OBJS)

test: $(TARGET)
	./test.sh

.PHONY: all clean test debug