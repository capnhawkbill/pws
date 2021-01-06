#!/bin/sh

rm -r build
mkdir build

# Compile backend
cd backend || exit
cargo build

cd .. || exit
cp backend/target/debug/server build/

# Make database
./backend/target/debug/db init build/db.sqlite

# Compile frontend
