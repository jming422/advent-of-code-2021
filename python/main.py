#!/usr/bin/env python3

import solvers.daythree


def read_input(i):
    with open(f"../puzzles/{i}-input.txt") as f:
        return f.readlines()


PUZZLE = 3


def main():
    lines = read_input(PUZZLE)
    out = solvers.daythree.part_two(lines)
    print(out)


if __name__ == "__main__":
    main()
