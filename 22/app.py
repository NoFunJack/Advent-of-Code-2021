import sys
import re

import numpy as np


class OffsetCube:
    def __init__(self,bounds):
        self.offset = (bounds[0],bounds[2],bounds[4])
        self.upper_edge = (bounds[1],bounds[3],bounds[5])
        off = [
            self.upper_edge[0]-self.offset[0]+1,
            self.upper_edge[1]-self.offset[1]+1,
            self.upper_edge[2]-self.offset[2]+1,
        ]
        self.cube = np.ones(off,dtype=np.int8)

    def substact(self,bounds):
        low,up = self.subcube_of(bounds)
        if low:
            self.cube[low[0]:up[0]+1,low[1]:up[1]+1,low[2]:up[2]+1] = 0

    def subcube_of(self,global_bounds):
        low_edge = [max(global_bounds[i*2],self.offset[i]) for i in range(0,3)]
        up_edge = [min(global_bounds[i*2+1],self.upper_edge[i]) for i in range(0,3)]
        low_edge = [low_edge[i]-self.offset[i] for i in range(0,3)]
        up_edge = [up_edge[i]-self.offset[i] for i in range(0,3)]
        return low_edge,up_edge


# Try to use numpy for the On-blocks
on_cubes = []


def add_cubes(bounds):
    for c in on_cubes:
        c.substact(bounds) 
    on_cubes.append(OffsetCube(bounds))

def remove_cubes(bounds):
    for c in on_cubes:
        c.substact(bounds) 

print("booting reactor\n")
p = re.compile("(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
with open(sys.argv[1], 'r') as file:
    for line in file:
        line =line.rstrip()
        print(line)
        m = p.match(line)
        bounds = [int(n) for n in m.groups()[1:7]]
        if m.group(1) == "on":
            add_cubes(bounds)
        else:
            remove_cubes(bounds)
        print(sum([off.cube.sum() for off in on_cubes]))




print("final lights",)
print(sum([off.cube.sum() for off in on_cubes]))


