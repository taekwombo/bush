#!/bin/sh

set -xe

STB=`pkg-config --cflags --libs stb`
RAYLIB="-L`pwd`/raylib/lib -I`pwd`/raylib/include -Wl,-R`pwd`/raylib/lib -lraylib"
LIBS="-lm -I`pwd` $STB $RAYLIB"

# Adder
gcc -Wall -Wextra -o ./demos/adder ./demos/adder.c $LIBS

# Vis
gcc -Wall -Wextra \
    -o ./demos/vis ./demos/vis.c \
    $LIBS


# Upscale
gcc -Wall -Wextra \
    -o ./demos/upscaling ./demos/upscaling.c \
    $LIBS
