#!/usr/bin/python
import re

def parse(sentence):
   return [int(item) for item in re.findall(r'\d+', sentence)]

def perm(x):
    if len(x) == 1:
        return [x]
    ret = []
    for y in x:
        perms = perm([item for item in x if item != y])
        for i in range(len(perms)):
            ret.append([y] + perms[i])
    return ret

def i_perm_of_n(i, n):
    return perm(range(n))[i-1]

while (True):
    cmd = raw_input()
    if cmd == "q":
        break
    nums = parse(cmd)
    print(i_perm_of_n(nums[0], nums[1]))

