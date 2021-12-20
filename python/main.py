#!/usr/bin/env python3

import solvers.dayseven


def read_input(i):
    with open(f"../puzzles/{i}-input.txt") as f:
        return f.readlines()


PUZZLE = 7


def main():
    lines = read_input(PUZZLE)
    out = solvers.dayseven.part_two(lines)
    print(out)


if __name__ == "__main__":
    main()
