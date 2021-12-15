def part_one(lines):
    horiz = 0
    depth = 0
    for line in lines:
        direction, magnitude = line.split(" ")
        delta = int(magnitude)
        if direction == "forward":
            horiz += delta
        elif direction == "up":
            depth -= delta
        elif direction == "down":
            depth += delta
        else:
            raise ValueError(f"Invalid direction {direction}")

    return horiz * depth


def part_two(lines):
    horiz = 0
    depth = 0
    aim = 0
    for line in lines:
        direction, magnitude = line.split(" ")
        delta = int(magnitude)
        if direction == "forward":
            horiz += delta
            depth += aim * delta
        elif direction == "up":
            aim -= delta
        elif direction == "down":
            aim += delta
        else:
            raise ValueError(f"Invalid direction {direction}")

    return horiz * depth
