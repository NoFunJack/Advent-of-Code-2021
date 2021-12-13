def printDots(dots):
    out = ""
    for y in range(max(dots,key=lambda x:x[1])[1]+1):
        for x in range(max(dots)[0]+1):
            if (x,y) in dots:
                out += "#"
            else:
                out += "."
        out += "\n"
    print(out)

def fold(dots, foldInst):
    if foldInst[0] == "y":
        return foldY(dots,foldInst[1])
    if foldInst[0] == "x":
        return foldX(dots,foldInst[1])

def foldY(dots,foldy):
    print(f"~~~folding on Y:{foldy}~~~\n")
    nDots = {x for x in dots if x[1]<foldy}
    
    overlap = {(x,2*foldy-y) for (x,y) in dots if y>foldy}
    return nDots|overlap

def foldX(dots,foldx):
    print(f"~~~folding on X:{foldx}~~~\n")
    nDots = {x for x in dots if x[0]<foldx}
    
    overlap = {(2*foldx-x,y) for (x,y) in dots if x>foldx}
    return nDots|overlap

import sys

with open(sys.argv[1]) as file:
    dots = set()
    while (line := file.readline()):
        line = line.strip()
        if not line:
            break;
        data = line.split(",")
        dots.add((int(data[0]),int(data[1])))
    
    folds =[]
    while (line := file.readline()):
        data = line.strip().split("fold along ")[1]
        data = data.split("=")
        folds.append((data[0],int(data[1])))


printDots(dots)
if sys.argv[2]=="part1":
    result = fold(dots,folds[0])
    printDots(result)
    print(f"num dots: {len(result)}")

if sys.argv[2]=="part2":
    for f in folds:
        dots = fold(dots,f)

    printDots(dots)
    print(f"num dots: {len(dots)}")



