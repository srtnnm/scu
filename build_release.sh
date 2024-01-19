#!/bin/bash 

release_build_dir=release_build
mkdir -p $release_build_dir


if [ $# -eq 0 ]
  then
    echo "No arguments supplied. version was not changed"
    release_build_dir+="/current"
    mkdir -p $release_build_dir
  else
    if [[ "$1" =~ ^[0-9]\.[0-9]\.[0-9]$ ]]
    then
      release_build_dir+="/$1"
      mkdir -p $release_build_dir
    else
      echo "$1 not matching X.X.X"
    fi
fi

mkdir -p logs

echo "building for x86_64" && cargo build --release --target=x86_64-unknown-linux-gnu &> logs/x86_64-unknown-linux-gnu.log && mv target/x86_64-unknown-linux-gnu/release/scu $release_build_dir/scu_glibc_x86_64 && cargo clean
echo "building for x86" && cargo build --release --target=i686-unknown-linux-gnu &> logs/i686-unknown-linux-gnu.log && mv target/i686-unknown-linux-gnu/release/scu $release_build_dir/scu_glibc_x86 && cargo clean


# maybe i'll do this later
# cargo build --release --target=aarch64-unknown-linux-gnu &> aarch64-unknown-linux-gnu.log && mv target/aarch64-unknown-linux-gnu/release/scu $release_build_dir/scu_glibc_arm64 && cargo clean
# cargo build --release --target=armv7-linux-androideabi &> armv7-linux-androideabi.log && mv target/armv7-linux-androideabi/release/scu $release_build_dir/scu_androideabi_armv7