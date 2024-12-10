map = []

with open("input") as f:
    for line in f:
        map.append([int(x) for x in line.strip()])

w = len(map[0])
h = len(map)


def step(row, col, cur_height):
    if cur_height == 9:
        return [(row, col)]
    ret = []
    if row > 0 and map[row - 1][col] == cur_height + 1:
        ret += step(row - 1, col, cur_height + 1)
    if row < h - 1 and map[row + 1][col] == cur_height + 1:
        ret += step(row + 1, col, cur_height + 1)
    if col > 0 and map[row][col - 1] == cur_height + 1:
        ret += step(row, col - 1, cur_height + 1)
    if col < w - 1 and map[row][col + 1] == cur_height + 1:
        ret += step(row, col + 1, cur_height + 1)
    return ret


sum = 0
sum_distinct = 0
for row in range(h):
    for col in range(w):
        if map[row][col] != 0:
            continue
        trails = step(row, col, 0)
        sum += len(set(trails))
        sum_distinct += len(trails)

print(sum)
print(sum_distinct)
