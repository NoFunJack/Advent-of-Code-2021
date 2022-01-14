import sys

# load players
p_pos = [int(pos) for pos in sys.argv[1:3]]
for i,pos in enumerate(p_pos):
    print(f"Player {i} starting position: {pos}")

# dice 
dice = 0
dice_rolled = 0
def inc_dice():
    global dice
    global dice_rolled
    dice_rolled += 1
    dice = (dice %100) +1
    return dice


def roll():
    v = [inc_dice() for i in range(1,4)]
    s = sum(v)
    print(f"rolled {v} sum: {s}")
    return s

points = [0,0]
won = False
while not won:
    for i,p in enumerate(p_pos):
        r = roll() 
        p = ((p + r) % 10) 
        if p ==0:
            p = 10
        points[i] += p
        p_pos[i] = p
        print(f"Player {i} rolls {r} to space {p} to a total score of {points[i]}")
        if points[i] >= 1000:
            won = True
            break

print(f"Result: {min(points)}*{dice_rolled} = {min(points)*dice_rolled}")
