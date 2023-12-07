filename='inputday3.txt'

def find_numbers(s, index):
    results=[]
    for size in range(10,0, -1):
        for i in range(len(s)-size+1):
            substring= s[i:i+size]
            if substring.isdigit():
                results.append((int(substring), i, i+size-1, index))
                s = s.replace(substring, '.'*size, 1)
    return results

def is_adjacent(lines, number):
    def check_line(line, number):
        left = 1 if number[1] != 0 else 0
        right = 1 if number[2] != len(line) else 0
        check_area=set( line[number[1]-left : number[2]+right+1] )
        return check_area & set('*#+$/&%-@=')

    result = False 
    result = True if check_line(lines[number[3]], number) else result 
    result = True if number[3] > 0 and check_line(lines[number[3]-1], number) else result
    result = True if number[3] < len(lines) - 1 and check_line(lines[number[3]+1], number) else result
    return result

def find_gears(s, index):
    results=[]
    for i in range(len(s)-2):
        substring= s[i:i+1]
        if substring == "*":
            results.append((substring, i, i, index))
            s = s.replace(substring, '.', 1)
    return results

def gear_adjacent(gear, numbers):
    def get_coords(number): return set([(x, number[3]) for x in range(number[1], number[2]+1)])
    count = 0
    adjacent=[]
    potential = set([(x, y) for x in range(gear[1]-1,gear[1]+2) for y in range(gear[3]-1,gear[3]+2)])
    for number in numbers:
        if get_coords(number) & potential:
            count += 1
            adjacent.append(number)
    if count == 2:
        return adjacent 

numbers, lines, gears = [], [], []
total, prod = 0, 0

with open(filename, 'r') as file:
    for i, line in enumerate(file):
        numbers.extend(find_numbers(line.strip(), i))
        lines.append(line.strip())

for _ in numbers:
    total = total + _[0] if is_adjacent(lines, _) else total

print(f"Part 1: {total}")

with open(filename, 'r') as file:
    for i, line in enumerate(file):
        gears.extend(find_gears(line.strip(), i))

for _ in gears:
    curr = gear_adjacent(_, numbers) 
    if curr:
        prod += curr[0][0] * curr[1][0]

print(f"Part 2: {prod}") 