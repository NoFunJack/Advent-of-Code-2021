class Image:
    def __init__(self,mapstr,algoArr):
        self.__parse_mapstr(mapstr)
        self.algoArr = algoArr
        self.__update_bounds()
        self.out_bounds_state = False

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
        for x in range(self.x_min-1,self.x_max+2):
            for y in range(self.y_min-1,self.y_max+2):
                if self.__getsig(x,y) in self.algoArr:
                    n.append((x,y))
        self.map=n
        self.__inc_bounds()
        if 0 in self.algoArr:
            self.out_bounds_state = not self.out_bounds_state

    def __getsig(self,x,y):
        bitstr =""
        for j in range(-1,2):
            for i in range(-1,2):
                if (x+i,y+j) in self.map:
                    bitstr += "1"
                else:
                    if self.__is_out_of_bounds(x+i,y+j):
                        if self.out_bounds_state:
                            #print("oob",x+i,y+j,self.out_bounds_state)
                            bitstr += "1"
                        else:
                            bitstr += "0"
                    else:
                        bitstr += "0"

        #print(x,y,"\t",bitstr)
        return int(bitstr,2)

    def __update_bounds(self):
        self.x_min= self.__min_pos(0)
        self.y_min= self.__min_pos(1)
        self.x_max= self.__max_pos(0)
        self.y_max= self.__max_pos(1)

    def __inc_bounds(self):
        self.x_min -= 1
        self.y_min -= 1
        self.x_max += 1
        self.y_max += 1

    def __is_out_of_bounds(self,x,y):
        return x < self.x_min or x > self.x_max or y < self.y_min or y > self.y_max

    def __min_pos(self,dim):
        if self.map:
            return min(self.map,key=lambda v: v[dim])[dim];
        else:
            return 0
        
    def __max_pos(self,dim):
        if self.map:
            return max(self.map,key=lambda v: v[dim])[dim];
        else:
            return 0

    def __str__(self):
        s = f"out of bounds state: {self.out_bounds_state}\n"
        s += f"bounds({self.x_min},{self.x_max},{self.y_min},{self.y_max})\n\n"
        for y in range(self.y_min,self.y_max+1):
            for x in range(self.x_min,self.x_max+1):
                if (x,y) in self.map:
                    s += '#'
                else:
                    s += '.'
            s += "\n"

        return s

