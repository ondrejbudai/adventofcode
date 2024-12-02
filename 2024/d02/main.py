safe = 0
safe_after_removal = 0

def is_safe(numbers):
    if numbers != sorted(numbers) and numbers != sorted(numbers, reverse=True):
        return False
    for a, b in zip(numbers[:-1], numbers[1:]):
        if abs(b - a) >= 1 and abs(b - a) <= 3:
            continue
        return False
    return True

with open('input') as f:
    for line in f:
        numbers = list(map(int, line.split()))
        if is_safe(numbers):
            safe += 1
        else:
            for i in range(len(numbers)):
                new_numbers = numbers[:i] + numbers[i+1:]
                if is_safe(new_numbers):
                    safe_after_removal += 1
                    break

print(safe)
print(safe + safe_after_removal)
