#!/bin/bash
# LEVEL 0
cargo run
# LEVEL 1
cd L1
mkdir -p out
mkdir -p bin
nasm -f elf64 -g -F dwarf -o ./out/main.o ./main.asm
ld ./out/main.o -o ./bin/main
cd bin
./main
cd ..
cd ..
# LEVEL 2
cd L2
mkdir -p out
mkdir -p bin
nasm -f elf64 -g -F dwarf -o ./out/main.o ./main.asm
ld ./out/main.o -o ./bin/main
cd bin
./main