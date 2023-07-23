#!/bin/sh

set -xe

clang -Wall -Wextra $@ -o adder adder.c -lm
