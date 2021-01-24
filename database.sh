#!/bin/sh

rm -r build/dist
rm -r build/server

# Compile backend
cd backend || exit
cargo build || exit

cd .. || exit
cp backend/target/debug/server build/

# Make database
# ./backend/target/debug/db init build/db.sqlite || exit

# Compile frontend
cd frontend || exit

npm run build || exit

cd .. || exit

cp -r frontend/dist build
