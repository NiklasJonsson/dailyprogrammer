#!/usr/bin/python
import re

def parse(sentence):
   return [int(item) for item in re.findall(r'\d+', sentence)]

def factoradic_of(dec):
    ret = []
    k = 1
    while True: 
        ret.append(dec % k)
        dec = dec / k
        if dec == 0:
            break
        k += 1
    return ret[::-1]

def i_perm_of_n(i, n):
    fac = factoradic_of(i-1)
    fac = [0 for x in range(n-len(fac))] + fac
    num_list = range(n)
    perm = []
    for digit in fac:
        perm.append(num_list.pop(digit))
    return perm

while (True):
    cmd = raw_input()
    if cmd == "q":
        break
    nums = parse(cmd)
    print(i_perm_of_n(nums[0], nums[1]))
