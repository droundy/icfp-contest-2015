#!/bin/sh

cargo build --release

for x in problems/*.json; do
    srun -J $x ./run.py -f $x -t 1300 --solver bottomupdfs --save &
done
