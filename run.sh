#!/bin/sh

export RUST_LOG=info

cd build || exit

./server
