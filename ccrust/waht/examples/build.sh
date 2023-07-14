#!/bin/sh

set -xe

clang -Wall -Wextra -o double   ./double.c && \
clang -Wall -Wextra -o gates    ./gates.c && \
clang -Wall -Wextra -o xor      ./xor.c 
