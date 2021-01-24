#!/bin/sh

if [[ "$1" == "--trace" ]]; then
  export RUST_LOG=trace
else
  export RUST_LOG=info
fi

cd build || exit

./server
