#!/usr/bin/bash

cargo new day$1
cd "day$1"
mkdir inputs
cd inputs
touch in1.txt
touch test1.txt
cd ../src
touch part1.rs
touch part2.rs
cat ../../rust_setup/main.rs > main.rs
cat ../../rust_setup/part.rs > part1.rs
cat ../../rust_setup/part.rs > part2.rs