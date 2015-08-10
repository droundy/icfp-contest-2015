#!/usr/bin/env python2
import sys, subprocess

with open("pop") as f:
    phrases = f.readlines()

phrase_args = []
for phrase in phrases:
    phrase_args += ["-p"]
    phrase_args += [phrase.rstrip("\n")]

cmd = ["./target/release/solve-davar"] + phrase_args + sys.argv[1:]


print("Running with:" + " ".join(cmd))
run = subprocess.Popen(cmd)
run.communicate()
