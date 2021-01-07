#!/bin/sh

export RUST_LOG=trace

cd build || exit

./server
