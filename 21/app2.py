import sys

# load players
start_pos = [int(pos) for pos in sys.argv[1:3]]
for i,pos in enumerate(start_pos):
    print(f"Player {i} starting position: {pos}")

weights = {
    3: 1,
    4: 3,
    5: 6,
    6: 7,
    7: 6,
    8: 3,
    9: 1,
}

games_won = [0,0]

def explore(points,pos,pix,num_games_base):
    if check_won(points,num_games_base):
        return
    for roll,num_games in weights.items():
        loop_points = points[:]
        loop_pos = pos[:]
        n_pos = (loop_pos[pix] + roll) % 10
        if n_pos == 0:
            n_pos = 10
        loop_points[pix] += n_pos
        loop_pos[pix] = n_pos
        #print(f"player {pix} rolled {roll} and moved to {n_pos} he now has {loop_points} points")
        explore(loop_points,loop_pos,next_player_id(pix),num_games_base*num_games)
        


def check_won(points,num_games):
    for i,p in enumerate(points):
        if p >= 21:
            games_won[i] += num_games
            #print(f"player {i} won {num_games} games with {p} points! Score {games_won}\n")
            return True
    return False

def next_player_id(old):
    if old == 1:
        return 0
    else:
        return 1

pos = start_pos[:]
explore([0,0],pos,0,1)

print(f"Final result: {games_won}\nmost wins {max(games_won)}")
