#!/bin/sh

set -xe

gcc -Wall -Wextra -I../ -o double   ./double.c   -lm && \
gcc -Wall -Wextra -I../ -o gates    ./gates.c    -lm && \
gcc -Wall -Wextra -I../ -o xor      ./xor.c      -lm && \
gcc -Wall -Wextra -I../ -o nero_xor ./nero_xor.c -lm     
