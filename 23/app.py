import sys
import copy

MOVE_COST = {"A":1,"B":10,"C":100,"D":1000}
DEST_ROOM = {"A":0,"B":1,"C":2,"D":3}

class State:
    def __init__(self,rooms): 
        self.hallway = [None for i in range(0,11)]
        self.room_pos = [i for i in range(2,9,2)]
        self.rooms = [[c for c in r] for r in rooms]
        self.used_energy = 0
        self.parent = None
        self.debug = "init_state"

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
        print(self.debug)

        if self.parent:
            print("||Parent")
            self.parent.print()
    

    def is_valid(self):
        pod_count = len([p for p in self.hallway if p])
        for r in self.rooms:
            if len(r)!= 2:
                return False
            pod_count += len([p for p in r if p])

        return pod_count == 8

    def is_done(self):
        return all(
                [all([c == t for c in self.rooms[i]]) for i,t in enumerate(["A","B","C","D"])]
                )

    def get_child_states(self):
        re = self.__get_to_hw_states()
        re += self.__get_into_room_states()
        # check
        for p in re:
            if not p.is_valid():
                print(f"\n\n+++Error in")
                p.print()
                assert False
        return re

    def __get_to_hw_states(self):
        re = []
        for i,r in enumerate([r for r in self.rooms if r]):
            clone = self.clone(f"PRE: move to hw {i}")
            if pod := clone.__move_first_from_room(i):
                re += clone.__move_left_from_room(i,pod)
                re += clone.__move_right_from_room(i,pod)
                
        return re

    def __get_into_room_states(self):
        re = []
        for i,p in enumerate(self.hallway):
            if p:
                clone = self.clone(f"move into room from hwpos {i}")
                clone = clone.__move_into_room(i,p)
                if clone:
                    re.append(clone)
        return re

    def __move_first_from_room(self,room_nr):
        if self.hallway[self.room_pos[room_nr]]:
            return None
        pod = None
        if self.rooms[room_nr][0]:
            pod = self.rooms[room_nr][0]
            if DEST_ROOM[pod] == room_nr and DEST_ROOM[self.rooms[room_nr][1]] == room_nr:
                return None
            self.rooms[room_nr][0] = None
            self.__add_cost(pod)
        elif self.rooms[room_nr][1]:
            pod = self.rooms[room_nr][1]
            if DEST_ROOM[pod] == room_nr:
                return None
            self.rooms[room_nr][1] = None
            self.__add_cost(pod,2)
        return pod

    def __move_left_from_room(self,room_nr,pod):
        return self.__move_in_hw(room_nr,pod,lambda p: p > 0,lambda p: p-1)

    def __move_right_from_room(self,room_nr,pod):
        return self.__move_in_hw(room_nr,pod,lambda p: p < 10,lambda p: p+1)

    def __move_in_hw(self,room_nr,pod,cond,pos_change):
        pos = self.room_pos[room_nr]
        print(room_nr,pos)
        init_pos = pos
        re = []
        while cond(pos):
            pos = pos_change(pos)
            if not self.hallway[pos]:
                c = self.clone(f"moved {pod} from room {room_nr} to pos: {pos}")
                c.__add_cost(pod,abs(init_pos-pos))
                c.hallway[pos]=pod
                re.append(c)
            else:
                break
        return re

    def __move_into_room(self,hw_pos,pod):
        room_idx = DEST_ROOM[pod]
        clone = self.clone(f"moved {pod} from hwpos: {hw_pos} to room {room_idx}")
        room = clone.rooms[room_idx]
        door_pos = clone.room_pos[room_idx]
        # check room contents
        if any([p and p != pod for p in room]):
            return False
        
        # if hw free
        for i in range(min(hw_pos,door_pos),max(hw_pos,door_pos)):
            if self.hallway[i]:
                return False

        clone.__add_cost(pod, abs(hw_pos-door_pos))
        clone.hallway[hw_pos] = None
        if not room[0]:
            room[0] = pod
            clone.__add_cost(pod)
        else:
            room[1] = pod
            clone.__add_cost(pod,2)

        return clone




    def __add_cost(self,pod,i=1):
        self.used_energy += MOVE_COST[pod]*i

    def clone(self,debugmsg):
        c = copy.deepcopy(self)
        c.parent = self
        c.debug = debugmsg
        return c

    def __eq__(self,other):
        return self.used_energy == other.used_energy and\
            self.hallway == other.hallway and \
            self.rooms[0] == other.rooms[0] and \
            self.rooms[1] == other.rooms[1] and \
            self.rooms[2] == other.rooms[2] and \
            self.rooms[3] == other.rooms[3] 

    def __hash__(self):
        re = 0
        for i,p in enumerate(self.hallway):
            if p:
                re += MOVE_COST[p]*(i+1)
        return re
            
init = State(sys.argv[1:5])
stl = {init}

smallest_solution = None

while stl:
    print(f"\nsmallest solution: {smallest_solution} loaded states: {len(stl)}")
    min_s = min(stl,key=lambda s:s.used_energy)
    print("smallest state")
    smallest_solution = min_s.used_energy
    if min_s.is_done():
        min_s.print()
        print(f"found solution with {smallest_solution} energy!")
        stl = {s for s in stl if s.used_energy < smallest_solution}
        break

    stl.remove(min_s)
    stl |= set(min_s.get_child_states())

    #for s in stl:
    #    print("\n" + "="*50)
    #    s.print()

print("done: ", smallest_solution)

