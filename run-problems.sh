#!/bin/sh

cargo build --release

for x in problems/*.json; do
    srun -J $x ./run.py -f $x --solver mc -t 360 -c 1 --submit --save &
done
