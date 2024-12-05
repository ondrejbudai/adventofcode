import functools

with open("input") as f:
    lines = f.readlines()

rules = []
for i, line in enumerate(lines):
    if not line.strip():
        break
    rules.append(line.strip().split("|"))

def sort_fn(a, b):
    for rule in rules:
        if rule[0] == a and rule[1] == b:
            return -1
        if rule[0] == b and rule[1] == a:
            return 1
    raise Exception("No rule found for %s, %s" % (a, b))


i += 1
correct_sum = 0
incorrect_sum = 0
for line in lines[i:]:
    pages = line.strip().split(",")
    sorted_pages = sorted(pages,key=functools.cmp_to_key(sort_fn))
    value = int(sorted_pages[len(sorted_pages) // 2])
    if sorted_pages != pages:
        incorrect_sum += value
    else:
        correct_sum += value


print(correct_sum)
print(incorrect_sum)

