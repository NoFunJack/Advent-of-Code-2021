#!/usr/bin/python

import sys

closer = {'(':')','[':']','{':'}','<':'>'}

with open(sys.argv[1]) as file:
    while (line := file.readline().rstrip()):
        print(line)
