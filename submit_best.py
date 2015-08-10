#!/usr/bin/env python2
import subprocess
import shlex
import re
import json
import glob

API_TOKEN = 'FtpwGAy9ndcLXLUlH7i96rgXLgi2SzEdym2caXEsNUI='
TEAM_ID = '97'

out_files = glob.glob("solutions/*.json")

out_files = sorted(out_files)

bases = [re.match( r'(solutions/[0-9]+\-[0-9]+)\-.*\.json', f).group(1) for f in out_files]

trimmed_bases = list(set(bases))

fnames = []
for base in trimmed_bases:
    possibilities = glob.glob(base + "*")
    scores = [re.match( r'.*\-([0-9]+)\.json', p).group(1) for p in possibilities]
    scores_num = sorted([int(s) for s in scores])

    winner = base + "-" + str(scores_num[-1]) + ".json"

    fnames += [winner]



raw_list = [open(fname).read() for fname in fnames]

json_list = [json.loads(raw) for raw in raw_list]

j = [item for sublist in json_list for item in sublist]

f = open("output.json", "w")

f.write(json.dumps(j))

f.close()

fname = 'output.json'

post_cmd = "curl --user :" + str(API_TOKEN) + " -X POST -H \"Content-Type: application/json\" " + \
           "-d @" + fname + " https://davar.icfpcontest.org/teams/" + str(TEAM_ID) + "/solutions"
print post_cmd

split_cmd = shlex.split(post_cmd)

subprocess.Popen(split_cmd)
