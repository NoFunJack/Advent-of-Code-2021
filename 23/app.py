import sys

class State:
    def __init__(self,rooms): 
        self.hw = [None for i in range(0,11)]
        self.room_pos = [i for i in range(2,9,2)]
        self.rooms = [[c for c in r] for r in rooms]

    def __tile_to_str(self,c):
        if not c:
            return "."
        else:
            return c

    def print(self):
        print("".join([self.__tile_to_str(c) for c in self.hw]))
        roomsStr = "  "\
                + " ".join([self.__tile_to_str(self.rooms[i][0]) for i in range(0,4)])\
                +"\n  "\
                + " ".join([self.__tile_to_str(self.rooms[i][1]) for i in range(0,4)])
        print(roomsStr)

    def is_done(self):
        return all(
                [all([c == t for c in self.rooms[i]]) for i,t in enumerate(["A","B","C","D"])]
                )


s = State(sys.argv[1:5])
s.print()
print("done: ", s.is_done())
