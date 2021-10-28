#!/usr/bin/env python3
# From here https://github.com/Dandigit/sloc
import os
import sys


class Flags():
    def __init__(self, root, types):
        self.root = root
        self.types = types


def getFlags():
    root = "./"
    types = []

    index = 1
    while index < len(sys.argv):
        arg = sys.argv[index]

        if arg == "-d" and len(sys.argv) > index + 1:
            root = sys.argv[index + 1]
            index += 1
        else:
            types.append(arg)

        index += 1

    sys.stdout.write("Searching for types: ")
    for type in types:
        sys.stdout.write(type + " ")

    print(f"in directory tree {root}.")

    return Flags(root, types)


def countLinesInFile(fileName):
    try:
        with open(fileName, "r", encoding="utf-8") as file:
            for i, l in enumerate(file, 1):
                pass
    except (UnicodeDecodeError, UnicodeEncodeError):
        print(
            f"Warning: did not process file: {fileName} as valid Unicode text.")
        return 0

    print(f"Counted {i} lines in file {fileName}.")
    return i


def getTotalLines(flags):
    def isCounted(name):
        for type in flags.types:
            if name.endswith("." + type) or type == "all":
                return True
        return False

    totalLines = 0
    for root, dirs, files in os.walk(flags.root):
        for file in files:
            if isCounted(file):
                totalLines += countLinesInFile(os.path.join(root, file))

    return totalLines


totalLines = getTotalLines(getFlags())
print(f"\nTOTAL: Counted {totalLines} lines of code.")
