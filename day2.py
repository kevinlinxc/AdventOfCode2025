with open('input2.txt', 'r') as file:
    lines: list[str] = file.readlines()

lines = [line.strip() for line in lines]
# strip removes whitespace 
ranges_text = lines[0]
# split splits a string
ranges = ranges_text.split(",")

total = 0

for range_str in ranges:
    start, end = range_str.split("-")
    start = int(start)
    end = int(end)
    print(start, end)
    for value in range(start, end + 1):
        value_str = str(value)
        str_len = len(value_str)
        first_half = value_str[:str_len//2]
        second_half = value_str[str_len//2:]

        if first_half == second_half:
            total += value
            
print(total)

