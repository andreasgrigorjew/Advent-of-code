#!/bin/python
import sys

def p1(maps, seeds):
    result = 2**31
    
    for s in seeds:
        v = s
        for m in maps:
            for (dst, src, rang) in m:
                if v >= src and v <= src + rang:
                    v = dst + v - src
                    break
        result = min(result, v)

    assert result < 2**31
    return result

def p2(maps, seeds):
    result = 2**31

    def intersecting(u, v, a, b):
        return (u >= a and u <= b) or (a >= u and a <= v) or (v >= a and v <= b) or (b >= u and b <= v)
    
    for (s,r) in zip(seeds[::2], seeds[1::2]):
        queue = [(s,s+r)]
        for m in maps:
            nq = []
            while queue:
                (u, v) = queue.pop()
                found_mapping = False
                for (dst, src, rang) in m:
                    if intersecting(u, v, src, src+rang):
                        found_mapping = True
                        L = max(u, src)
                        R = min(v, src+rang)
                        nq.append((dst + L - src, dst + R - src))
                        if L > u:
                            queue.append((u, L-1))
                        if R < v:
                            queue.append((R+1, v))
                        break
                if not found_mapping:
                    nq.append((u, v))
            queue = nq
        for (l, r) in queue:
            result = min(result, l)


    assert result < 2**31
    return result

seeds = []

maps = []
for _ in range(7):
    maps.append([])
map_names = ["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"]
currently_reading = -1

for line in sys.stdin:
    words = line.split()
    if not words:
        continue
    if words[0] == "seeds:":
        seeds = list(map(int, words[1:]))
        continue
    if words[0] in map_names:
        currently_reading = map_names.index(words[0])
        assert currently_reading >= 0 and currently_reading < 7
    else:
        maps[currently_reading].append((int(words[0]), int(words[1]), int(words[2])))

print(p1(maps, seeds))
print(p2(maps, seeds))
