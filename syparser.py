import re

def stdin():
    return int(input("> "))

def _process_line(idx: int, line: str, vars: dict, leafs: dict) -> int:
    op = line.split(" ")[0]
    args = line.split(" ")[1:]

    if op == "sy" and (len(args) != 2 and len(args) != 4):
        raise Exception(f"Command `sy` (idx {idx}) must be given exactly 2 or 4 arguments.")
    elif op == "leaf" and (len(args) != 1):
        raise Exception("Command `leaf` must be given exactly 1 argument.")
    elif op != "sy" and op != "leaf":
        raise Exception(f"Unrecognized command `{op}`.")

    if op == "sy":
        return _sy(idx, args, vars, leafs)
    return idx + 1


def _sy(idx, args, vars: dict, leafs: dict):
    try:
        op1 = float(args[0])
    except:
        if args[0] in vars:
            op1 = vars[args[0]]
        elif args[0] == "stdin":
            op1 = stdin()
        else:
            raise Exception("`sy` requires that arguments one and two be numbers capable of being converted to floats.")
    try:
        op2 = float(args[1])
    except:
        if args[1] in vars:
            op1 = vars[args[1]]
        elif args[1] == "stdin":
            op2 = stdin()
        else:
            raise Exception("`sy` requires that arguments one and two be numbers capable of being converted to floats.")
    var_name = None if (args[2] if len(args) == 4 else None) == "_" else (args[2] if len(args) == 4 else None)
    leaf_name = None if (args[3] if len(args) == 4 else None) == "_" else (args[3] if len(args) == 4 else None)
    
    diff = op1 - op2
    if var_name != None:
        if var_name != "stdout":
            vars[var_name] = diff
        else:
            print(chr(int(diff)), end='')
    if diff <= 0 and leaf_name != None:
        try:
            return leafs[leaf_name]
        except KeyError:
            raise Exception(f"Leaf `{leaf_name}` not found in script.")
    else:
        return idx + 1


class Parser():
    def __init__(self):
        self.data = ""
        self.vars = dict()
        self.leafs = dict()

    def from_file(self, filename: str) -> None:
        with open(filename, "r") as f:
            self.data = f.read()

    def from_string(self, data: str) -> None:
        self.data = data

    def run(self) -> dict:
        lines = self.data.split(';')
        lines = [l.strip() for l in lines]
        # Cull comments
        lines = [re.sub(r'<<.+>>', "", l) for l in lines]
        lines = [l.strip() for l in lines]

        for idx, line in enumerate(lines):
            line = line.split(" ")
            if line[0] == "leaf":
                self.leafs[line[1]] = idx

        i = 0
        while True:
            try:
                line = lines[i]
            except IndexError:
                break
            if line != "":
                i = _process_line(i, line, self.vars, self.leafs)
            else:
                i += 1

        return self.vars, self.leafs