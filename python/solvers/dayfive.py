def is_horiz_or_vert(line):
    return line[0]["x"] == line[1]["x"] or line[0]["y"] == line[1]["y"]


def parse(input_line):
    return [
        {"x": int(x), "y": int(y)}
        for x, y in [s.strip().split(",") for s in input_line.split("->")]
    ]


def do_it(input_lines, diagonals=False):
    lines = [
        line
        for line in [parse(input_line) for input_line in input_lines]
        if diagonals or is_horiz_or_vert(line)
    ]
    max_x = max(point["x"] for line in lines for point in line)
    max_y = max(point["y"] for line in lines for point in line)
    plot = [[0 for _ in range(max_x + 1)] for _ in range(max_y + 1)]
    for a, b in lines:
        cur_x = a["x"]
        cur_y = a["y"]
        dest_x = b["x"]
        dest_y = b["y"]
        plot[cur_y][cur_x] += 1
        while cur_x - dest_x != 0 or cur_y - dest_y != 0:
            if cur_x < dest_x:
                cur_x += 1
            elif cur_x > dest_x:
                cur_x -= 1
            if cur_y < dest_y:
                cur_y += 1
            elif cur_y > dest_y:
                cur_y -= 1
            plot[cur_y][cur_x] += 1
    return sum(1 for row in plot for point in row if point > 1)


def part_one(lines):
    return do_it(lines)


def part_two(lines):
    return do_it(lines, True)
