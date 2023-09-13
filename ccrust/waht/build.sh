#!/bin/sh

set -xe

PWD=`pwd`

# Adder
# ADDER_FLAGS="-DBITS=4 -DEPOCHS=1000"
# gcc -Wall -Wextra "$ADDER_FLAGS" -o ./demos/adder ./demos/adder.c -lm -I`pwd`

RAYLIB="-lraylib -I$PWD/raylib/include -I$PWD/raylib/lib"
NERO="-I$PWD"

# Vis
gcc -Wall -Wextra \
    -o ./demos/vis ./demos/vis.c \
    -lm \
    -I`pwd` \
    -I`pwd`/raylib/include \
    -L`pwd`/raylib/lib \
    -Wl,-R`pwd`/raylib/lib \
    -lraylib \
    -DPLOT_ENTRIES=1000 \
    -DRAD=10 -DBITS=4 $@

PNG=`pkg-config --cflags --libs libpng`

# Upscale
gcc -Wall -Wextra $@ \
    -o ./demos/upscaling ./demos/upscaling.c \
    -lm \
    -I`pwd`/raylib/include \
    -L`pwd`/raylib/lib \
    -Wl,-R`pwd`/raylib/lib \
    -lraylib \
    -lpng \
    -I`pwd`
