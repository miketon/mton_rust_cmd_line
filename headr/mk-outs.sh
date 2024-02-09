#!/usr/bin/env bash

# @audit : test zsh vs bash ... do both output  pass unit test?
# debug to inspect when we run with bash v zsh
echo "This script is running in bash, version $BASH_VERSION"

INPUTS="./tests/inputs"
OUT_DIR="./tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $INPUTS/*.txt; do
  BASENAME=$(basename "$FILE")
  head $FILE > ${OUT_DIR}/${BASENAME}.out
done
