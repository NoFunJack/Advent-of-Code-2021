
class CaveMap:
    def __init__(self,lines):
        self. connections = []
        for line in lines:
            parts = line.strip().split("-")
            self.connections.append((parts[0],parts[1]))
            self.connections.append((parts[1],parts[0]))

    def findConnections(self,n):
        return [x[1] for x in self.connections if x[0]==n]
            

assert(len(CaveMap([]).connections)==0)
assert(len(CaveMap(["a-b"]).connections)==2)
assert(CaveMap(["a-b"]).findConnections("a")==["b"])
assert(CaveMap(["a-b","a-c"]).findConnections("a")==["b","c"])
assert(CaveMap(["a-b","a-c"]).findConnections("b")==["a"])

class Navi:
    def __init__(self,myMap):
        self.myMap = myMap

    def findfrom(self,start):
        paths = [[start]]
        return self._findFrom(paths)

    def _findFrom(self,paths):
        assert(paths)
        newPaths = []
        for p in paths:
            if p[-1] != "end":
                np = self._addNext(p)
                if np:
                    newPaths.extend(self._findFrom(np))
            else:
                newPaths.append(p)
        return newPaths


    def _addNext(self,l):
        re = []
        for n in self.myMap.findConnections(l[-1]):
            if n.isupper() or self._canAddLowercase(l,n):
                re.append(l+[n])
        return re

    def _canAddLowercase(self,l,c):
        if c == "start":
            return False
        if c == "end":
            return True
        if c not in l:
            return True
        return not any( x.islower() and l.count(x)>1 for x in l )


# load file
import sys

with open(sys.argv[1]) as file:
    caveMap = CaveMap(file.readlines())

for i,v in enumerate(sorted(Navi(caveMap).findfrom("start"))):
    print(i+1,",".join(v))

