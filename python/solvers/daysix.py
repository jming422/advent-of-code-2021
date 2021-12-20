def part_one(lines):
    fish = [int(f) for f in lines[0].split(",")]
    for _ in range(80):
        fish = [f - 1 for f in fish]
        for i, f in enumerate(fish):
            if f < 0:
                fish[i] = 6
                fish.append(8)
    return len(fish)


# The solution from part_one takes WAY too much time & memory; it will
# practically never finish! So we have to do something cleverer. I got some
# help on this one from my brilliant wife and from
# https://barretblake.dev/blog/2021/12/advent-of-code-day6/, but the code
# itself I still wrote without copying.
def part_two(lines):
    # Indices 0-8 are the fish states, values are the number of fish in each
    # state
    fish = [0] * 9
    for f in lines[0].split(","):
        fish[int(f)] += 1

    for _ in range(256):
        # fish.pop(0) will shift all the values left one index for us!
        reproducing_fish = fish.pop(0)
        fish[6] += reproducing_fish
        fish.append(reproducing_fish)

    return sum(fish)
