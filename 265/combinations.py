#!/usr/bin/python
import re

def parse(sentence):
    return [int(item) for item in re.findall(r'\d+', sentence)]

def comb(k, nlist):
    if k == 0:
        return [[]]
    ret = []
    for i in range(len(nlist)):
        combs = comb(k-1, nlist[i+1:])
        for c in combs:
            c.insert(0,nlist[i])
            ret.append(c)
    return ret

def gen_comb(i, k, n):
    return comb(k,range(n))[i-1]

while (True):
    cmd = raw_input()
    if cmd == "q":
        break
    nums = parse(cmd)
    print(gen_comb(nums[0], nums[1], nums[2]))
