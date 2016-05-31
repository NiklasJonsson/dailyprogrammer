#!/usr/bin/python
import re

def parse(sentence):
   return [int(item) for item in re.findall(r'\d+', sentence)]

def comb(k, n):
    for i in range(n):
        for j in range(


def (i, k, n):
    return comb

while (True):
    cmd = raw_input()
    if cmd == "q":
        break
    nums = parse(cmd)
    print(i_perm_of_n(nums[0], nums[1]))

