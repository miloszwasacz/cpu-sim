#!/bin/sh

FILE_PATH="$1"
if [ ! -f "$FILE_PATH" ]; then
    echo "File '$FILE_PATH' not found"
    exit 1
fi

DIR=$(dirname "$FILE_PATH")
FILE=$(basename "$FILE_PATH")
FILENAME=$(echo "$FILE" | cut -d. -f1)
OBJ="$DIR/$FILENAME.o"
BIN="$DIR/$FILENAME.bin"

echo "Assembling $FILE..."
as "$FILE" -o "$OBJ"

echo "Preparing binary..."
objcopy -O binary "$OBJ" "$BIN"

echo "Cleaning up..."
rm "$OBJ"

echo "Done"
