#!/bin/bash

cd ../
cargo build --release
upx -9 -q -q -q target/release/spl

mv target/release/spl release