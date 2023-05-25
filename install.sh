#!/bin/bash

cargo build --release
if [[ -f "target/release/scu" ]]; then
  echo "Please write root password for installing program."
  sudo mv target/release/scu /usr/bin
else
  echo "Compilation failed"
fi
