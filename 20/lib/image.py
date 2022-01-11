class Image:
    def __init__(self,mapstr,algoArr):
        self.__parse_mapstr(mapstr)
        self.algoArr = algoArr

    def __parse_mapstr(self,mapstr):
        lines = mapstr.split("\n")
        self.map = []
        for y,line in enumerate(lines):
            for x, c in enumerate(line):
                if c == '#':
                    self.map.append((x,y))

        pass

    def enhance(self):
        n = []
        for x in range(self.__min_pos(0)-1,self.__max_pos(0)+2):
            for y in range(self.__min_pos(1)-1,self.__max_pos(1)+2):
                if self.__getsig(x,y) in self.algoArr:
                    n.append((x,y))
        self.map=n

    def __getsig(self,x,y):
        bitstr =""
        for j in range(-1,2):
            for i in range(-1,2):
                if (x+i,y+j) in self.map:
                    bitstr += "1"
                else:
                    bitstr += "0"

        return int(bitstr,2)


    def __min_pos(self,dim):
        return min(self.map,key=lambda v: v[dim])[dim];
        
    def __max_pos(self,dim):
        return max(self.map,key=lambda v: v[dim])[dim];

    def __str__(self):
        s = ""
        for y in range(self.__min_pos(1)-1,self.__max_pos(1)+2):
            for x in range(self.__min_pos(0)-1,self.__max_pos(0)+2):
                if (x,y) in self.map:
                    s += '#'
                else:
                    s += '.'
            s += "\n"

        return s
