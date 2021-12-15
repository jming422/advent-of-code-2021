def mode_digit(count):
    if count["ones"] >= count["zeros"]:
        return 1
    else:
        return 0


def count_bits(bin_strs):
    acc = [{"zeros": 0, "ones": 0} for _ in range(len(bin_strs[0]))]
    for bin_str in bin_strs:
        for i, char in enumerate(bin_str):
            if char == "0":
                digit = "zeros"
            elif char == "1":
                digit = "ones"
            else:
                raise ValueError("That isn't binary!")
            acc[i][digit] += 1
    return acc


def part_one(lines):
    counts = count_bits([line.strip() for line in lines])
    num_shifts = len(counts) - 1
    gamma = 0
    epsilon = 0
    for position, digit in enumerate(map(mode_digit, counts)):
        gamma |= digit << (num_shifts - position)
        epsilon |= (digit ^ 1) << (num_shifts - position)
    return gamma * epsilon


def count_digit(bin_strs, i):
    zeros = 0
    ones = 0
    for s in bin_strs:
        if s[i] == "0":
            zeros += 1
        elif s[i] == "1":
            ones += 1
        else:
            raise ValueError("That isn't binary!")
    return zeros, ones


def part_two(lines):
    o2_strs = [line.strip() for line in lines]
    bit_length = len(o2_strs[0])
    co2_strs = o2_strs.copy()
    for i in range(bit_length):
        zeros, ones = count_digit(o2_strs, i)
        o2_strs = [s for s in o2_strs if s[i] == ("1" if ones >= zeros else "0")]
        if len(o2_strs) <= 1:
            break
    for i in range(bit_length):
        zeros, ones = count_digit(co2_strs, i)
        co2_strs = [s for s in co2_strs if s[i] == ("1" if ones < zeros else "0")]
        if len(co2_strs) <= 1:
            break

    return int(o2_strs[0], 2) * int(co2_strs[0], 2)
