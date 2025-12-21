with open('input2.txt', 'r') as file:
    lines: list[str] = file.readlines()

lines = [line.strip() for line in lines]
# strip removes whitespace 
ranges_text = lines[0]
# split splits a string
ranges = ranges_text.split(",")

total = 0

def check_rest(substring, full_string):
    sub_length = len(substring)
    full_length = len(full_string)
    # First check if the full string length is divisible by substring length
    if full_length % sub_length != 0:
        return False
    for i in range(0, full_length, sub_length):
        to_compare = full_string[i:i+sub_length]
        if to_compare != substring:
            return False
    return True


for range_str in ranges:
    start, end = range_str.split("-")
    start = int(start)
    end = int(end)
    for value in range(start, end + 1):
        value_str = str(value)
        str_len = len(value_str)
        for substring_length in range(1, str_len//2 + 1):
            substring = value_str[:substring_length]
            if check_rest(substring, value_str):
                total += value
                break
            
print(total)
