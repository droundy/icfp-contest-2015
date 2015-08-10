#!/bin/sh

set -e

echo $1
TIME=30

test -e problems/problem_$1.json

(sleep 2 && ./play_icfp2015 -p 'ei!' -p 'Ia! Ia!' -p "R'lyeh" -p Yuggoth -p yogsothoth -p 'planet 10' -p 'john bigboote' -t $TIME -c 1 --animate 40 -f problems/problem_$1.json && killall ffmpeg -SIGINT && echo Finished!) &

for i in `seq 1 100`; do
    echo \$
done

echo \$ echo Solving problem_$1.json in $TIME seconds.
echo Running problem_$1.json in $TIME seconds.

echo \$ ./play_icfp2015 -p 'ei!' -p "'Ia! Ia!'" -p "R'lyeh" -p Yuggoth -p yogsothoth -p "'planet 10'" -p "'john bigboote'" -t $TIME -c 2 --animate 40 -f problems/problem_$1.json &

./play_icfp2015 -p 'ei!' -p 'Ia! Ia!' -p "R'lyeh" -p Yuggoth -p yogsothoth -p 'planet 10' -p 'john bigboote' -t $TIME -c 1 -f problems/problem_$1.json > /dev/null

ffmpeg -f x11grab -show_region 1 -y -r 25 -s 1280x720 -i :0.0+0,40 -vcodec ffv1 screencast_$1.avi 2> /dev/null
