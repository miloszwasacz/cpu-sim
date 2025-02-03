#!/bin/bash

FILE_PATH="$1"
if [ ! -f "$FILE_PATH" ]; then
    echo "File '$FILE_PATH' not found"
    exit 1
fi

DIR=$(dirname "$FILE_PATH")
FILE=$(basename "$FILE_PATH")
FILENAME=$(echo "$FILE" | cut -d. -f1)
BIN="$DIR/$FILENAME.bin"

echo "Compiling $FILE..."
# To see the linker script, add the `-Wl,--verbose` option
riscv64-unknown-elf-gcc -mabi=ilp32 -march=rv32i -no-pie -nostdlib -e main -O1 -o "$BIN" "$FILE_PATH"

echo "Done"
