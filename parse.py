#!/usr/bin/env python2
import os

splitLen = 200         # 200 lines per file
outputBase = 'parsed/output' # output.1.txt, output.2.txt, etc.

# This is shorthand and not friendly with memory
# on very large files (Sean Cavanagh), but it works.
input = open('metadata.json', 'r').read().split('}')

#NOTE: Parses forwards, look at the highest file number for most recent
if os.path.exists('parsed'):
    os.system("rm -r parsed")
os.system("mkdir -p parsed")

at = 1
for lines in range(0, len(input), splitLen):
    # If file doesn't exist or if last 
    # First, get the list slice
    outputData = input[lines:lines+splitLen]
    
    # Now open the output file, join the new slice with newlines
    # and write it out. Then close the file.
    output = open(outputBase + str(at) + '.dat', 'w')
    output.write('\n\n'.join(outputData))
    output.close()

    # Increment the counter
    at += 1
        
