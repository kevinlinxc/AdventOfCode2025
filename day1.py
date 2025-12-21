import fileinput

lines = [line.strip() for line in fileinput.input(files="inputs/1.txt")]

lock_state = 50

num_zeros = 0
for line in lines:
    direction = line[0]
    value = int(line[1:])
    if direction == "R":
        lock_state += value
    elif direction == "L":
        lock_state -= value
    lock_state = lock_state % 100
    if lock_state == 0:
        num_zeros += 1

print(f"Final lock state: {lock_state}")
print(f"Number of times lock state was zero: {num_zeros}")