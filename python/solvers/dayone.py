def solve(lines):
    count = 0
    prev = None
    for line in lines:
        num = int(line.strip())
        if prev and prev < num:
            count += 1
        prev = num

    return count
