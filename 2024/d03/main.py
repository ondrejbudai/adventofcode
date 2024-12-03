# example of how to do regex search in Python

import re

regex = r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))"

with open("input") as f:
    data = f.read()

matches = re.findall(regex, data)
sum_all = 0
sum_cond = 0
enabled = True
for match in matches:
    if match[3] == "do()":
        enabled = True
    elif match[4] == "don't()":
        enabled = False
    else:
        sum_all += int(match[1]) * int(match[2])
        if enabled:
            sum_cond += int(match[1]) * int(match[2])

print(sum_all)
print(sum_cond)
