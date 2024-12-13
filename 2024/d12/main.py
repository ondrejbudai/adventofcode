map = []

with open("input") as f:
    for line in f:
        map.append(list(line.strip()))

visited = set()

def in_bounds(row, col):
    return 0 <= row < len(map) and 0 <= col < len(map[0])

dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]

# returns (area, perimeter)
def visit(row, col):
    if (row, col) in visited:
        return 0, 0

    visited.add((row, col))

    area = 1
    perimeter = 0
    current = map[row][col]

    for dir in dirs:
        new_row, new_col = row + dir[0], col + dir[1]
        if in_bounds(new_row, new_col) and map[new_row][new_col] == current:
            a, p = visit(new_row, new_col)
            area += a
            perimeter += p
        else:
            perimeter += 1

    return area, perimeter

sum = 0

for row in range(len(map)):
    for col in range(len(map[0])):
        a, p = visit(row, col)
        sum += a * p

print(sum)


