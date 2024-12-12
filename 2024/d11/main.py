import functools

with open("input") as f:
    stones = [int(d) for d in f.read().strip().split()]


@functools.cache
def blink_stone(value, remaining_blinks):
    if remaining_blinks == 0:
        return 1
    if value == 0:
        return blink_stone(1, remaining_blinks - 1)
    value_str = str(value)
    if len(value_str) % 2 == 0:
        left = value_str[: len(value_str) // 2]
        right = value_str[len(value_str) // 2 :]
        return blink_stone(int(left), remaining_blinks - 1) + blink_stone(
            int(right), remaining_blinks - 1
        )
    return blink_stone(2024 * value, remaining_blinks - 1)


sum = 0
for stone in stones:
    sum += blink_stone(stone, 25)

print(sum)

sum = 0
for stone in stones:
    sum += blink_stone(stone, 75)

print(sum)
