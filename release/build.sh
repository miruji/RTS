#!/bin/bash
# build spl x86-32 or x86-64
# and optimize (strip + upx)

cd ../
cargo build --release #--target=i686-unknown-linux-gnu

strip target/release/spl
upx --best -q -q -q target/release/spl

mv target/release/spl release