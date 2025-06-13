# SPDX-License-Identifier: Apache-2.0
# Simple Makefile for quick builds

CC = cc
CFLAGS = -O3 -Wall -Wextra -std=c99 -Iinclude
LDFLAGS =

SRCS = src/main.c src/gitmind.c src/link.c src/sha1.c src/path.c src/check.c src/status.c
OBJS = $(SRCS:.c=.o)
TARGET = gitmind

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