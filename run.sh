#!/bin/sh

export RUST_LOG=info

(cd frontend && exec npm run serve) &
build/server
