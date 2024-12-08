import itertools

antennas = {}

with open("input") as f:
    for row, line in enumerate(f):
        for col, char in enumerate(line.strip()):
            if char == ".":
                continue
            ant_type = antennas.get(char, [])
            ant_type.append((row, col))
            antennas[char] = ant_type


def in_bounds(pos):
    return 0 <= pos[0] <= row and 0 <= pos[1] <= col


antinodes = set()
antinodes_with_harmonics = set()
for ant_type in antennas:
    for a, b in itertools.permutations(antennas[ant_type], 2):
        d_row, d_col = a[0] - b[0], a[1] - b[1]
        pos = a
        while in_bounds(pos):
            antinodes_with_harmonics.add(pos)
            pos = (pos[0] + d_row, pos[1] + d_col)

        if in_bounds((a[0] + d_row, a[1] + d_col)):
            antinodes.add((a[0] + d_row, a[1] + d_col))

print(len(antinodes))
print(len(antinodes_with_harmonics))
