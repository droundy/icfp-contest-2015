#!/usr/bin/env python2
import sys, subprocess

with open("pop") as f:
    phrases = f.readlines()

phrase_args = []
for phrase in phrases:
    phrase_args += ["-p"]
    phrase_args += [phrase.rstrip("\n")]

cmd = ["./target/release/solve-davar"] + phrase_args + sys.argv[1:]

buildcmd = ["cargo", "build", "--release"]
print("Building with: " + " ".join(buildcmd))

build = subprocess.Popen(buildcmd)
build.communicate()

if build.returncode == 0:
    print("Running with:" + " ".join(cmd))
    run = subprocess.Popen(cmd)
    run.communicate()
