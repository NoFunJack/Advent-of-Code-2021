#!/usr/bin/python

class Dumbos:    
    def __init__(self,lines):
        self.h = len(lines)
        self.w = len(lines[0].strip())
        self.data = self._phase(lines)

        self.flashcount = 0
        self.flasher = []

    def _phase(self,lines):
        data = []
        for s in lines:
           s = s.strip() 
           for pw in s:
               data.append(int(pw))
        return data

    def print(self):
        for i in range(self.h):
            start = self.w*i
            end = self.w*(i+1)
            print("".join(str(x) for x in self.data[start:end]))
        print("\nflashcount:",self.flashcount)

    def step(self):
        self._addOne()
        self._flashAll()
        self._resetFlasher()

    def areAllFlashing(self):
        return all(v==0 for v in self.data)

    def _addOne(self):
        self.data = [i+1 for i in self.data]

    def _flashAll(self):
        someflashed = True
        while someflashed:
            someflashed = False
            for i,v in enumerate(self.data):
                if v > 9 and (i not in self.flasher): 
                    self._flash(i)
                    someflashed = True

    def _flash(self,idx):
        self.flasher.append(idx)
        self.flashcount += 1

        for i in neighIdx(idx,self.w,self.h):
            self.data[i] += 1

    def _resetFlasher(self):
        for idx in self.flasher:
            self.data[idx] = 0

        self.flasher = []
        
def neighIdx(idx,w,h):
    neigh = []
    isLeftEdge = idx % w == 0
    isRightEdge = idx % w == w-1

    # ups
    if idx > w-1:
        up = idx-w 
        neigh.append(up)
        if not isLeftEdge:
            neigh.append(up-1)
        if not isRightEdge:
            neigh.append(up+1)

    # middles
    if not isLeftEdge:
        neigh.append(idx-1)

    if not isRightEdge:
        neigh.append(idx+1)
    
    # downs
    if idx < (h-1)*w:
        down = idx+w 
        neigh.append(down)
        if not isLeftEdge:
            neigh.append(down-1)
        if not isRightEdge:
            neigh.append(down+1)

    return neigh 

# unit Tests

assert set(neighIdx(0,1,1)) == set([])
assert set(neighIdx(0,2,1)) == set([1])
assert set(neighIdx(0,1,2)) == set([1])
assert set(neighIdx(0,2,2)) == set([1,2,3])
assert set(neighIdx(4,3,3)) == set([0,1,2,3,5,6,7,8])
assert set(neighIdx(10,10,10)) == set([0,1,11,20,21])

# === working code ===
import sys

with open(sys.argv[1]) as file:
    dumbos = Dumbos(file.readlines())

param = sys.argv[2]

if param == "part2":
    step = 0
    while not dumbos.areAllFlashing():
        print("\n\nstep:",step)
        step += 1
        dumbos.print()
        dumbos.step()
    print("final step:",step)
    dumbos.print()
else:
    for i in range(int(sys.argv[2])+1):
        print("\n\nstep:",i)
        dumbos.print()
        dumbos.step()
