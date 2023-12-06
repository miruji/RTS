#!/bin/bash

rustc -C prefer-dynamic ../src/Main.rs -o Main
upx -9 -q -q -q Main