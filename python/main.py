#!/usr/bin/env python3

import solvers.dayone


def read_input(i):
    with open(f"../puzzles/{i}-input.txt") as f:
        return f.readlines()


PUZZLE = 1


def main():
    lines = read_input(PUZZLE)
    out = solvers.dayone.solve(lines)
    print(out)


if __name__ == "__main__":
    main()
