#!/usr/bin/env python

import json

raw_json = input = open('metadata.json', 'r').read()

results_list = json.loads(raw_json)

results_list = sorted(results_list, key=lambda d: d[u'createdAt'])

print "Listing five most recent results"
for my_dict in results_list:
    print "Score: " + str(my_dict[u'score'])
    print "Tag: " + my_dict[u'tag']
    print "Solution: " + my_dict[u'solution']
    print "Powerscore: " + str(my_dict[u'powerScore'])
    print "Seed: " + str(my_dict[u'seed'])
    print "Problem: " + str(my_dict[u'problemId'])
    print "Timestamp: " + my_dict[u'createdAt']
    date_str = my_dict[u'createdAt']
    day = date_str[8:10]
    time = date_str[11:20]
    print "Day: " + day
    print "Time: " + time
    print ""
