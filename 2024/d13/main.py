import re

with open("input") as f:
    data = f.read()

# YOLO way to find groups of 6 numbers
matches = re.findall(r"\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)", data)


def find_cheapest(adx, ady, bdx, bdy, px, py):
    b = (py * adx - ady * px) / (bdy * adx - ady * bdx)
    a = (px - b * bdx) / adx

    if not a.is_integer() or not b.is_integer():
        return 0

    return a * 3 + b


sum = 0
sum2 = 0

for match in matches:
    adx, ady, bdx, bdy, px, py = map(int, match)
    sum += find_cheapest(adx, ady, bdx, bdy, px, py)
    sum2 += find_cheapest(adx, ady, bdx, bdy, px + 10000000000000, py + 10000000000000)

print(sum)
print(sum2)
