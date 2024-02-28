#!/usr/bin/env bash

ROOT="tests/inputs"
FILES="$ROOT/empty.txt $ROOT/fox.txt $ROOT/atlamal.txt"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $FILES; do
  BASENAME=$(basename "$FILE")
  wc $FILE > ${OUT_DIR}/${BASENAME}.out
done

wc < "$ROOT/atlamal.txt" > "$OUT_DIR/atlamal.txt.stdin.out"

wc $FILES > $OUT_DIR/all.out
