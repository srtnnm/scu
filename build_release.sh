#!/bin/bash 

cargo build --release && mkdir -p release_build && mv target/release/scu release_build/scu_glibc_x86_64
