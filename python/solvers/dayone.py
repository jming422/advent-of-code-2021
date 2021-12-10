def part_one(lines):
    count = 0
    prev = None
    for line in lines:
        num = int(line.strip())
        if prev and prev < num:
            count += 1
        prev = num

    return count


def part_two(lines):
    count = 0
    nums = [int(line) for line in lines]
    window = nums[:3]
    prev = sum(window)
    for num in nums[3:]:
        window.pop(0)
        window.append(num)
        val = sum(window)
        if prev < val:
            count += 1
        prev = val

    return count
