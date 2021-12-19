#!/usr/bin/env python3

import solvers.dayfour


def read_input(i):
    with open(f"../puzzles/{i}-input.txt") as f:
        return f.readlines()


PUZZLE = 4


def main():
    lines = read_input(PUZZLE)
    out = solvers.dayfour.part_two(lines)
    print(out)


if __name__ == "__main__":
    main()
