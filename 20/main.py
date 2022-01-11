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
imagemap.enhance()
print("first enhancment")
print(imagemap)
imagemap.enhance()
print("second enhancment")
print(imagemap)
print("pips: ", len(imagemap.map))
