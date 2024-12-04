dirs = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
]
needle = "XMAS"

map = []

with open("input") as f:
    for line in f:
        map.append(list(line.strip()))

count = 0

for row in range(len(map)):
    for col in range(len(map[0])):
        for dir in dirs:
            for i in range(len(needle)):
                row1 = row + i * dir[0]
                col1 = col + i * dir[1]
                if row1 < 0 or row1 >= len(map) or col1 < 0 or col1 >= len(map[0]):
                    break
                if map[row1][col1] != needle[i]:
                    break
                if i == len(needle) - 1:
                    count += 1
print(count)

count = 0
dirs = [[(-1, -1), (1,1)], [(1, -1), (-1, 1)], [(-1, 1), (1, -1)], [(1, 1), (-1, -1)]]
needle = ["M", "S"]

for row in range(len(map)):
    for col in range(len(map[0])):
        if map[row][col] != "A":
            continue
        matches = 0
        for dir in dirs:
            for i in range(len(needle)):
                row1 = row + dir[i][0]
                col1 = col + dir[i][1]
                if row1 < 0 or row1 >= len(map) or col1 < 0 or col1 >= len(map[0]):
                    break
                if map[row1][col1] != needle[i]:
                    break
                if i == len(needle) - 1:
                    matches += 1
        if matches == 2:
            count += 1

print(count)





