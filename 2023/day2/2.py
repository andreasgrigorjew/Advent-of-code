#!/bin/python
import fileinput

def part1():
    R = 12
    G = 13
    B = 14

    res = 0
    for (j, line) in enumerate(fileinput.input()):
        words = line.split()
        poss = True
        for (i, w) in enumerate(words):
            if w.startswith("red") and int(words[i-1]) > R:
                poss = False
            if w.startswith("green") and int(words[i-1]) > G:
                poss = False
            if w.startswith("blue") and int(words[i-1]) > B:
                poss = False
        if poss:
            res += j+1

    return res

def part2():
    res = 0
    for line in fileinput.input():
        R = 0
        G = 0
        B = 0
        words = line.split()
        for (i, w) in enumerate(words):
            if w.startswith("red"):
                R = max(R, int(words[i-1]))
            if w.startswith("green"):
                G = max(G, int(words[i-1]))
            if w.startswith("blue"):
                B = max(B, int(words[i-1]))
        res += R*G*B

    return res

print(part2())
