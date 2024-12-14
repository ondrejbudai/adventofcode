import re
import sys

positions, velocities = [], []

for line in open("input"):
    x, y, dx, dy = map(int, re.findall(r"-?\d+", line))
    positions.append((x, y))
    velocities.append((dx, dy))

# w = 11
# h = 7
w = 101
h = 103


def calc_counts():
    robot_counts = {}
    for x, y in positions:
        robot_counts[(x, y)] = robot_counts.get((x, y), 0) + 1
    return robot_counts


def print_map():
    robot_counts = calc_counts()
    for y in range(h):
        for x in range(w):
            print(robot_counts.get((x, y), "."), end="")
        print()
    print()


if sys.argv[1] == "slideshow":
    seconds = 0
    while True:
        if (seconds - 33) % 101 == 0:
            print("\033[2J", end="")
            print(seconds)
            print_map()
            input()
        seconds += 1
        for i in range(len(positions)):
            x, y = positions[i]
            dx, dy = velocities[i]
            positions[i] = ((x + dx) % w, (y + dy) % h)


for _ in range(100):
    for i in range(len(positions)):
        x, y = positions[i]
        dx, dy = velocities[i]
        positions[i] = ((x + dx) % w, (y + dy) % h)

counts = calc_counts()
safety_factor = 1

sum = 0
for x in range(w // 2):
    for y in range(h // 2):
        sum += counts.get((x, y), 0)

safety_factor *= sum

sum = 0
for x in range(w // 2 + 1, w):
    for y in range(h // 2):
        sum += counts.get((x, y), 0)

safety_factor *= sum

sum = 0
for x in range(w // 2):
    for y in range(h // 2 + 1, h):
        sum += counts.get((x, y), 0)

safety_factor *= sum

sum = 0
for x in range(w // 2 + 1, w):
    for y in range(h // 2 + 1, h):
        sum += counts.get((x, y), 0)

safety_factor *= sum
print(safety_factor)
