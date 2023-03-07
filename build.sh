#!/bin/bash

cargo run
cd L1
mkdir -p out
mkdir -p bin
nasm -f elf64 -g -F dwarf -o ./out/main.o ./main.asm
ld ./out/main.o -o ./bin/main
cd bin
./main