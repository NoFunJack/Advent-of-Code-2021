import sys

from lib.image import Image

with open(sys.argv[1], 'r') as file:
    data = file.read().split("\n")

algstr = data.pop(0)
del(data[0])
del(data[-1])
data = '\n'.join(data)

alg = [pos for pos, char in enumerate(algstr) if char == '#']
imagemap = Image(data,alg)
print(imagemap)
for i in range(0,int(sys.argv[2])):
    imagemap.enhance()
    print(f"{i} enhancment")
    print(imagemap)
print("pips: ", len(imagemap.map))
