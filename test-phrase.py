#!/usr/bin/env python2
import sys, subprocess


cmd = ["./target/release/solve-davar"] + "--solver mc -t 10 -f problems/problem_0.json".split(" ") + ["-p", sys.argv[1], "--tag", sys.argv[1], "--submit"]

buildcmd = ["cargo", "build", "--release"]
print("Building with: " + " ".join(buildcmd))

build = subprocess.Popen(buildcmd)
build.communicate()

if build.returncode == 0:
    print("Running with:" + " ".join(cmd))
    run = subprocess.Popen(cmd)
    run.communicate()
