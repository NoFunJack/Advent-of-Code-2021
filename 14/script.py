import re

def phaseInput(line):
    re = {}
    for i in range(len(line)-1):
        key =line[i]+line[i+1]
        re[key] = re.get(key,0) +1
    return re


def applyInserts(mat,inserts):
    re = {}
    for pair,v in mat.items():
        if pair in inserts:
            keyL = pair[0]+inserts[pair]
            keyR = inserts[pair]+pair[1]
            re[keyL] = re.get(keyL,0) +v
            re[keyR] = re.get(keyR,0) +v
        else:
            re[pair] = re.get(pair,0)+1
    return re

# ab -> ab
assert(applyInserts({"ab":1},{})=={"ab":1})
# ab -> acb
assert(applyInserts({"ab":1},{"ab":"c"})=={"ac":1,"cb":1})
# abc -> acbc
assert(applyInserts({"ab":1,"bc":1},{"ab":"c"})=={"ac":1,"cb":1,"bc":1})
# aaaa -> acacaca
assert(applyInserts({"aa":3},{"aa":"c"})=={"ac":3,"ca":3})

import sys

with open(sys.argv[1]) as file:
    line =file.readline().strip()
    ends = (line[0],line[-1])
    mat = phaseInput(line)
    inserts = {} 
    # skip space
    file.readline()
    while (line := file.readline()):
        line = line.strip()
        match = re.search(r"(\w\w) -> (\w)",line)
        inserts[match.group(1)] =match.group(2)

print("Start: ",mat)

for _ in range(int(sys.argv[2])):
    mat = applyInserts(mat,inserts)
    
# calc score
counts = {}
counts[ends[0]] = 1
counts[ends[1]] = 1
for k,v in mat.items():
    counts[k[0]] = counts.get(k[0],0)+v
    counts[k[1]] = counts.get(k[1],0)+v



print(f"score is: {(max(counts.values())-min(counts.values()))/2}")

