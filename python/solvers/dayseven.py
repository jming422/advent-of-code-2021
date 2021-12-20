def fuel_cost(crabs, pos):
    return sum(abs(crab - pos) for crab in crabs)


def part_one(lines):
    nums = [int(s) for s in lines[0].split(",")]
    return min(fuel_cost(nums, num) for num in nums)


def fuel_cost_two(crabs, dest):
    return sum(sum(range(1, abs(crab - dest) + 1)) for crab in crabs)


def part_two(lines):
    crabs = [int(s) for s in lines[0].split(",")]
    min_crab = min(crabs)
    max_crab = max(crabs)
    return min(fuel_cost_two(crabs, dest) for dest in range(min_crab, max_crab + 1))
