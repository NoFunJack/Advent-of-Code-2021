import math

class cood:
    def __init__(self,value):
        self.value = int(value)
        self.dist = math.inf

def load(lines):
    m = []
    for line in lines:
        line = line.strip()
        # convert into (value,best dist, prev)
        line = [cood(c)  for c in line]
        m.append(line)

    return m

def printCave(cave):
    for line in cave:
        print([f"{c.value}[{c.dist}]" for c in line])

def dij(cave,start):
    (x,y) = start
    curr = cave[x][y]
    curr.dist = 0

    neigh = [(x,y+1),(x,y-1),(x-1,y),(x+1,y)]

    for ne in neigh:
        (x,y) = ne
        if not (0 <= x < len(cave) and 0 <= y < len(cave[x])):
                continue
        n = cave[x][y]

        d = curr.dist + n.value
        if d < n.dist:
            n.dist = d
            visitNeigh(cave,n,pos)
        


def visitNeigh(cave,curr,pos):
    (x,y) = pos
    visit(cave,curr,(x,y+1))
    visit(cave,curr,(x,y-1))
    visit(cave,curr,(x+1,y))
    visit(cave,curr,(x-1,y))





# load file
import sys

with open(sys.argv[1]) as file:
    cave = load(file.readlines())

dij(cave,(0,0))
printCave(cave)
print(cave[-1][-1].dist)


