#!/bin/sh

set -xe

if test ! -d ./raylib/; then
    wget https://github.com/raysan5/raylib/releases/download/5.0/raylib-5.0_linux_amd64.tar.gz -O raylib.tar.gz
    tar -xvzf raylib.tar.gz
    mv ./raylib-5.* ./raylib
fi

# fetch raylib if not here
RAYLIB="-L`pwd`/raylib/lib -I`pwd`/raylib/include -Wl,-R`pwd`/raylib/lib -lraylib"
LIBS="-lm -I`pwd` $RAYLIB"

gcc -Wall -Wextra \
    -o ./integration ./integration.c \
    $LIBS
