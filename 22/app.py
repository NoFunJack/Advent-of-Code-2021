import sys
import re

import numpy as np


class OffsetCube:
    def __init__(self,bounds):
        self.bounds = bounds

    def cut(self,bounds):
        old_lights = self.volume()
        re = []
        skip = False
        #print("\ncut",self.bounds,bounds)
        for i in range(0,6,2):
            new_cube = self.bounds[:]
            if bounds[i] > self.bounds[i]:
                #print("cut low",i)
                b = min(bounds[i]-1,self.bounds[i+1])
                new_cube[i+1] = b
                re.append(OffsetCube(new_cube))
                if b == self.bounds[i+1]:
                    #print("whole cube deleted",b)
                    skip = True
                    break
                self.bounds[i] = max(b+1,self.bounds[i])
                #print("new cube",new_cube,b)
                #print("remainder",self.bounds)
        if not skip:
            for i in range(1,6,2):
                new_cube = self.bounds[:]
                if bounds[i] < self.bounds[i]:
                    #print("cut high",i)
                    b = max(bounds[i]+1,self.bounds[i-1])
                    new_cube[i-1] = b
                    re.append(OffsetCube(new_cube))
                    if b == self.bounds[i-1]:
                        #print("whole cube deleted",b)
                        break
                    self.bounds[i] = min(b-1,self.bounds[i])
                    #print("new cube",new_cube,b)
                    #print("remainder",self.bounds)
                
        #print("new cubes",[c.bounds for c in re])
        diff = sum([off.volume() for off in re])-old_lights
        #print("diff:",diff)
        for c in re:
            assert c.volume() > 0
        assert diff<=0

        return re


    def volume(self):
        dim = [self.bounds[i*2+1]-self.bounds[i*2]+1 for i in range(0,3)]
        re = np.prod(dim)
        ##print(f"{self.bounds} dim: {dim} has volume of {re}")
        return re



# Try to use numpy for the On-blocks
on_cubes = []


def add_cubes(bounds):
    global on_cubes
    new_cubes = []
    for c in on_cubes:
        new_cubes.extend(c.cut(bounds))
    on_cubes = new_cubes
    on_cubes.append(OffsetCube(bounds))

def remove_cubes(bounds):
    global on_cubes
    new_cubes = []
    for c in on_cubes:
        new_cubes.extend(c.cut(bounds))
    on_cubes = new_cubes

print("booting reactor\n")
p = re.compile("(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
with open(sys.argv[1], 'r') as file:
    for lineNum,line in enumerate(file):
        line =line.rstrip()
        print(lineNum,line)
        m = p.match(line)
        bounds = [int(n) for n in m.groups()[1:7]]
        if m.group(1) == "on":
            add_cubes(bounds)
        else:
            remove_cubes(bounds)
        print("~~lights:",sum([off.volume() for off in on_cubes]))




print("final lights",)
print(sum([off.volume() for off in on_cubes]))


