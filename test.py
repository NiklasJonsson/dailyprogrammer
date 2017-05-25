#!/usr/bin/env python3
from sys import argv
from os import listdir
from os.path import isfile, join
import re
import sys
import os
import subprocess



folder_re = argv[1]
file_re = ""

if len(argv) == 3:
    file_re = argv[2]

dirs = [x for x in os.listdir(".") if re.search(folder_re, x)]

for dir in dirs:
    files = [x for x in os.listdir(dir) if re.search(file_re, x) and not re.search("\.", x)]
    for file in files:
        base_dir = "./" + dir + "/"
        exec_file = base_dir + file
        in_file = base_dir + file + ".in"
        ref_file = base_dir + file + ".ref"

        if os.path.isfile(in_file) and os.path.isfile(ref_file):
            cmd = ["./" + dir + "/" + file, dir + "/" + file + ".in"]
            sub = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            out,err = sub.communicate()
            out = out.decode("utf-8")
            with open(ref_file, 'r') as reff:
                ref = reff.read();
                print(exec_file, end=" ");
                if out == ref:
                    print("PASS");
                else:
                    print("FAIL");
                    print("REF:");
                    print(ref);
                    print("OUT:");
                    print(out);

