import sys
import copy

MOVE_COST = {"A":1,"B":10,"C":100,"D":1000}

class State:
    def __init__(self,rooms): 
        self.hallway = [None for i in range(0,11)]
        self.room_pos = [i for i in range(2,9,2)]
        self.rooms = [[c for c in r] for r in rooms]
        self.used_energy = 0

    def __tile_to_str(self,c):
        if not c:
            return "."
        else:
            return c

    def print(self):
        print(f"used energy: {self.used_energy}")
        print("".join([self.__tile_to_str(c) for c in self.hallway]))
        roomsStr = "  "\
                + " ".join([self.__tile_to_str(self.rooms[i][0]) for i in range(0,4)])\
                +"\n  "\
                + " ".join([self.__tile_to_str(self.rooms[i][1]) for i in range(0,4)])
        print(roomsStr)

    def is_done(self):
        return all(
                [all([c == t for c in self.rooms[i]]) for i,t in enumerate(["A","B","C","D"])]
                )

    def get_child_states(self):
        re = self.__get_to_hw_states()
        return re

    def __get_to_hw_states(self):
        re = []
        for i,r in enumerate([r for r in self.rooms if r]):
            clone = self.clone()
            pod = clone.__move_first_from_room(i)
            re += clone.__move_left_from_room(i,pod)
            re += clone.__move_right_from_room(i,pod)

    def __move_first_from_room(self,room_nr):
        if self.rooms[room_nr][0]:
            pod = self.rooms[room_nr][0]
            self.rooms[room_nr][0] = None
            self.__add_cost(pod)
        elif self.rooms[room_nr][1]:
            pod = self.rooms[room_nr][1]
            self.rooms[room_nr][1] = None
            self.__add_cost(pod,2)
        return pod



    def __move_left_from_room(self,room_nr,pod):
        return self.__move_in_hw(room_nr,pod,lambda p: p > 0,lambda p: p-1)

    def __move_right_from_room(self,room_nr,pod):
        return self.__move_in_hw(room_nr,pod,lambda p: p < 10,lambda p: p+1)

    def __move_in_hw(self,room_nr,pod,cond,pos_change):
        pos = self.room_pos[room_nr]
        init_pos = pos
        re = []
        while cond(pos) and not self.hallway[pos]:
            print(f"pos: {pos}")
            pos = pos_change(pos)
            c = self.clone()
            c.__add_cost(pod,abs(init_pos-pos))
            c.hallway[pos]=pod
            re.append(c)
            c.print()
        return re

    def __add_cost(self,pod,i=1):
        self.used_energy += MOVE_COST[pod]*i

    def clone(self):
        return copy.deepcopy(self)
            

s = State(sys.argv[1:5])
s.print()

for c in s.get_child_states():
    c.print()

print("done: ", s.is_done())
