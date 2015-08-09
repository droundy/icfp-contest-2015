#!/bin/sh

PHRASES=-p 'ei!' -p 'Ia! Ia!' -p "R'lyeh" -p Yuggoth

for x in problems/*.json; do
    srun -J $x target/release/solve-davar -f $x $PHRASES --solver mc -t 360 -c 1 --submit --save &
done
