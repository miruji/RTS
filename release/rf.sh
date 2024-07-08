#!/bin/bash
# build and run test
# e: [./test.sh <filename>] or debug
# e: [./test.sh <filename> -d]

clear

if [ ! -e "spl" ]; then
  ./build.sh
fi

./spl -rf "$@"