#!/bin/sh

cargo build --release

for x in problems/*.json; do
    srun -J $x target/release/solve-davar -f $x \
         -p 'ei!' -p 'Ia! Ia!' -p "R'lyeh" -p Yuggoth \
         --solver mc -t 360 -c 1 --submit --save &
done
