#!/bin/bash
cargo build --release
strip --strip-all target/release/smallest_hello_world
objcopy --strip-unneeded target/release/smallest_hello_world
ls -lh target/release/smallest_hello_world
