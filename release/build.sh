#!/bin/bash

cd ../
cargo build --release #--target=i686-unknown-linux-gnu

strip target/release/spl
upx --best -q -q -q target/release/spl

mv target/release/spl release