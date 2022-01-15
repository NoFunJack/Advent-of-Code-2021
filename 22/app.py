import numpy as np

import sys
import re

matrix_size = 250001
offset = 125000

cubes = np.zeros([matrix_size for i in range(0,3)],dtype=np.uint8)

def processNumber(nstr):
    return int(nstr)+50

print("booting reactor\n")
p = re.compile("(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
with open(sys.argv[1], 'r') as file:
    for line in file:
        line =line.rstrip()
        print(line)
        m = p.match(line)
        state = 1 if m.group(1) == "on" else 0
        bounds = [processNumber(n) for n in m.groups()[1:7]]
        print(bounds)
        if not any([b > matrix_size for b in bounds]):
            cubes[bounds[0]:bounds[1]+1,bounds[2]:bounds[3]+1,bounds[4]:bounds[5]+1] = state
            print("on states",cubes.sum()) 

print("final lights",cubes.sum())


