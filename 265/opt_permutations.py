#!/usr/bin/python
import re
import math

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

def decimal_of(fac_list):
    num = 0
    for i in range(1,len(fac_list)):
        num = num + math.factorial(i)*fac_list[-i-1]
    return num
        
    
def find_idx(elem, l):
    if l == []:
        l.append(elem)
        return 0
    i = 0
    while i < len(l) and elem > l[i]:
        i += 1
    l.insert(i, elem)
    return i

def perm_idx_of(perm):
    l = []
    fac_list = []
    perm = [int(x) for x in perm]
    for p in perm[::-1]:
        idx = find_idx(p,l)
        fac_list.append(idx)
    return decimal_of(fac_list[::-1])
'''
while (True):
    cmd = raw_input()
    if cmd == "q":
        break
    nums = parse(cmd)
    print(i_perm_of_n(nums[0], nums[1]))
'''
