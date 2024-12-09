orig_disk = []

with open("input") as f:
    data = f.readline().strip()
    for i, block in enumerate(data):
        block = int(block)
        if i % 2 == 0:
            # file
            id = i // 2
            for _ in range(block):
                orig_disk.append(id)
        else:
            for _ in range(block):
                orig_disk.append(None)

# fragment
disk1 = orig_disk.copy()
next_to_move = len(disk1) - 1
next_to_scan_for_none = 0

while True:
    while disk1[next_to_move] is None:
        next_to_move -= 1
    while disk1[next_to_scan_for_none] is not None:
        next_to_scan_for_none += 1
    if next_to_move <= next_to_scan_for_none:
        break
    disk1[next_to_scan_for_none] = disk1[next_to_move]
    disk1[next_to_move] = None
    next_to_move -= 1
    next_to_scan_for_none += 1


def checksum(d):
    sum = 0
    for i, id in enumerate(d):
        if id is None:
            continue
        sum += i * id
    return sum


print(checksum(disk1))

disk2 = orig_disk.copy()
i = len(disk2) - 1
while True:
    file = disk2[i]
    for j in range(i - 1, -1, -1):
        if disk2[j] != file:
            break
    start = j + 1
    size = i - start + 1
    if file is not None:
        space = 0
        for j in range(0, i):
            if disk2[j] is None:
                space += 1
                if space >= size:
                    for k in range(j - size + 1, j + 1):
                        disk2[k] = file
                    for k in range(start, start + size):
                        disk2[k] = None
                    break
            else:
                space = 0
    i -= size
    if i <= 0:
        break

print(checksum(disk2))
