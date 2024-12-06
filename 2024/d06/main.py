import copy

map = []

with open("input") as f:
    for row, line in enumerate(f):
        map_row = []
        map.append(map_row)

        for col, char in enumerate(line.strip()):
            if char == "^":
                start = (row, col)
                char = "."

            map_row.append(char)


turns = {"up": "right", "right": "down", "down": "left", "left": "up"}
delta = {"up": (-1, 0), "right": (0, 1), "down": (1, 0), "left": (0, -1)}


def is_valid(pos):
    row, col = pos
    return 0 <= row < len(map) and 0 <= col < len(map[row])


def simulate_guard():
    pos = start
    direction = "up"
    visited = set()
    while True:
        if (pos, direction) in visited:
            return visited, True  # found loop
        visited.add((pos, direction))
        row, col = pos
        new_row, new_col = row + delta[direction][0], col + delta[direction][1]
        if not is_valid((new_row, new_col)):
            return visited, False  # run away
        if map[new_row][new_col] == "#":
            direction = turns[direction]
            continue
        pos = (new_row, new_col)


# part 1
visited, _ = simulate_guard()
visited_without_direction = set((pos for pos, _ in visited))
print(len(visited_without_direction))

# part 2
loops = 0
for row in range(len(map)):
    for col in range(len(map[row])):
        # if not on the original path, they won't ever hit the new obstacle
        if (row, col) not in visited_without_direction:
            continue
        if map[row][col] == "#" or map[row][col] == "^":
            continue
        map[row][col] = "#"
        _, loop = simulate_guard()
        if loop:
            loops += 1
        map[row][col] = "."
print(loops)
