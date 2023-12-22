#!/usr/bin/env bash
# A special commment (aka shebang) that tells the os to
# use the environment to execute bash for the following
# code

# var for output directory
OUTDIR="tests/expected"
# tests if output directory exist
# - create it if needed
[[ ! -d "$OUTDIR" ]] && mkdir -p $OUTDIR

echo "Hello there" > $OUTDIR/args_1.txt
echo "Hello"  "there" > $OUTDIR/args_2.txt
echo -n "Hello  there" > $OUTDIR/args_1.n.txt
echo -n "Hello"  "there" > $OUTDIR/args_2.n.txt
