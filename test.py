#!/usr/bin/env python3
from sys import argv
from os import listdir
from os.path import isfile, join
import re
import sys
import os
import subprocess

def main():
    # Empty string matches anything
    folder_re = "\d+" if len(argv) == 1 else argv[1]
    file_re = ""

    if len(argv) == 3:
        file_re = argv[2]

    dirs = [x for x in os.listdir(".") if re.search(folder_re, x) and os.path.isdir(x)]

    for dir in dirs:
        files = [x for x in os.listdir(dir) if re.search(file_re, x) and re.search("\.rs", x)]
        for file in files:
            base = re.sub("\.rs", "", file)
            base_dir = "./" + dir + "/"
            exec_file = base_dir + base
            in_file = base_dir + base + ".in"
            ref_file = base_dir + base + ".ref"
            src_file = base_dir + base + ".rs"

            if os.path.isfile(exec_file):
                os.remove(exec_file)

            if os.path.isfile(src_file):
                cmd = ["rustc", src_file, "-o", exec_file]
                sub = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                out,err = sub.communicate()
                out = out.decode("utf-8")
                err = err.decode("utf-8")
                if sub.returncode != 0:
                    print(out)
                    print(err)
                    continue

            if os.path.isfile(in_file) and os.path.isfile(ref_file):
                cmd = [exec_file, in_file]
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

if __name__ == "__main__":
    main()

