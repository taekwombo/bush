#!/bin/sh

set -xe

# Adder
gcc -Wall -Wextra $@ -o ./demos/adder ./demos/adder.c -lm -I`pwd`

# Vis
gcc -Wall -Wextra $@ \
    -o ./demos/vis ./demos/vis.c \
    -lm \
    -I`pwd` \
    -I`pwd`/raylib/include \
    -L`pwd`/raylib/lib \
    -Wl,-R`pwd`/raylib/lib \
    -lraylib

./demos/vis
