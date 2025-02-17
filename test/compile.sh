#!/bin/bash
#TODO Move to Makefile

FILE_PATH="$1"
if [ ! -f "$FILE_PATH" ]; then
    echo "File '$FILE_PATH' not found"
    exit 1
fi

SCRIPT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" || exit 2 ; pwd -P )
DIR=$(dirname "$FILE_PATH")
FILE=$(basename "$FILE_PATH")
FILENAME=$(echo "$FILE" | cut -d. -f1)
IO_LIB_NAME="simpleio"
IO_LIB_OBJ="$SCRIPT_PATH/$IO_LIB_NAME.o"
BIN="$DIR/$FILENAME.bin"

if [ ! -f "$IO_LIB_OBJ" ]; then
  echo "Compiling $IO_LIB_NAME.c..."
  riscv64-unknown-elf-gcc -mabi=ilp32 -march=rv32i -fPIC -c -o "$IO_LIB_OBJ" "$SCRIPT_PATH/$IO_LIB_NAME.c"
fi

echo "Compiling $FILE..."
# To see the linker script, add the `-Wl,--verbose` option
riscv64-unknown-elf-gcc -mabi=ilp32 -march=rv32i -no-pie \
                        -flto -fuse-linker-plugin \
                        -fdata-sections -ffunction-sections -Wl,--gc-sections \
                        -O1 -o "$BIN" "$IO_LIB_OBJ" "$FILE_PATH"
#riscv64-unknown-elf-gcc -mabi=ilp32 -march=rv32i -no-pie -nostdlib -e main -O1 -o "$BIN" "$FILE_PATH"

echo "Done"
