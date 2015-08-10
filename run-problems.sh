#!/bin/sh

cargo build --release

for x in problems/*.json; do
    srun -J $x ./run.py -f $x --solver bottomupdfs --submit --save &
done
