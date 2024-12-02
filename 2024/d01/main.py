f = open("input", "r")
lines = f.readlines()
a = []
b = []
for line in lines:
    na, nb = line.split()
    a.append(int(na))
    b.append(int(nb))

a.sort()
b.sort()

sum = 0

for x, y in zip(a, b):
    sum += abs(x - y)

print(sum)

occurences = {}
for x in b:
    if x in occurences:
        occurences[x] += 1
    else:
        occurences[x] = 1

sum = 0

for x in a:
    sum += x * occurences.get(x, 0)

print(sum)

