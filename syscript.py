import sys

from syparser import Parser

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("syscript.py must be given exactly one argument - the filename to run.")
        exit(0)

    p = Parser()

    p.from_file(sys.argv[1])

    _, _ = p.run()