def step(cur, operands, result):
    if cur > result:
        return False

    if len(operands) == 0:
        return cur == result
    rest = operands[1:]
    return step(cur + operands[0], rest, result) or step(
        cur * operands[0], rest, result
    )


def step2(cur, operands, result):
    if cur > result:
        return False

    if len(operands) == 0:
        return cur == result
    rest = operands[1:]
    return (
        step2(cur + operands[0], rest, result)
        or step2(cur * operands[0], rest, result)
        or step2(int(str(cur) + str(operands[0])), rest, result)
    )


with open("input") as f:
    sum = 0
    sum_extra_op = 0
    for line in f:
        result, rest = line.strip().split(": ", 1)
        result = int(result)
        operands = [int(op) for op in rest.split()]
        if step(operands[0], operands[1:], result):
            sum += result

        if step2(operands[0], operands[1:], result):
            sum_extra_op += result

    print(sum)
    print(sum_extra_op)
