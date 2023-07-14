#!/bin/sh

set -xe

clang -Wall -Wextra $@ -o playground playground.c -lm
