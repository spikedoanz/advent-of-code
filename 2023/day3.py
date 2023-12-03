filename='day3.txt'

def find_numbers(s, index):
    results=[]
    size=10
    while size > 0:
        for i in range(len(s)-size+1):
            substring= s[i:i+size]
            if substring.isdigit():
                results.append((int(substring), i, i+size-1, index))
                s = s.replace(substring, '.'*size, 1)
        size -= 1
    return results

def is_adjacent(lines, number):
    def check_line(line, number):
        left = 1 if number[1] != 0 else 0
        right = 1 if number[2] != len(line) else 0
        check_area=line[number[1]-left : number[2]+right+1]
        for _ in check_area:
            if _ in '*#+$/&%-@=':
                return True
        return False 
    result = False 
    result = True if check_line(lines[number[3]], number) else result 
    result = True if number[3] > 0 and check_line(lines[number[3]-1], number) else result
    result = True if number[3] < len(lines) - 1 and check_line(lines[number[3]+1], number) else result
    return result

numbers=[]
lines=[]


### Part 1
with open(filename, 'r') as file:
    i=0
    for line in file:
        numbers.extend(find_numbers(line.strip(), i))
        i+=1
        lines.append(line.strip())
    

total=0
for _ in numbers:
    #print(_, is_adjacent(lines,_))
    if is_adjacent(lines, _):
        total+= _[0]
print(f"Part 1: {total}")



### Part 2
def find_gears(s, index):
    results=[]
    size=1
    while size > 0:
        for i in range(len(s)-size+1):
            substring= s[i:i+size]
            if substring == "*":
                results.append((substring, i, i+size-1, index))
                s = s.replace(substring, '.'*size, 1)
        size -= 1
    return results

def get_coords(number):
    return [(x, number[3]) for x in range(number[1], number[2]+1)] 
def gear_adjacent(gear, numbers):
    '''
    ...
    .*. [symbol, left, right, line]
    ...
    '''
    count = 0
    adjacent=[]
    potential = [
        (gear[1]-1, gear[3]-1), (gear[1], gear[3]-1), (gear[1]+1, gear[3]-1),
        (gear[1]-1, gear[3]),   (gear[1], gear[3]),   (gear[1]+1, gear[3]),
        (gear[1]-1, gear[3]+1), (gear[1], gear[3]+1), (gear[1]+1, gear[3]+1),
        ]
    #print(potential)
    for i, number in enumerate(numbers):
        coords = get_coords(number)
        for coord in coords:
            if coord in potential:
                count+=1
                adjacent.append(number)
                break 
        
    if count == 2:
        return adjacent 
    return []

gears = []
with open(filename, 'r') as file:
    i=0
    for line in file:
        gears.extend(find_gears(line.strip(), i))
        i+=1
        lines.append(line.strip())

prod = 0
for _ in gears:
    curr = gear_adjacent(_, numbers) 
    if curr != []:
        prod += curr[0][0] * curr[1][0]
print(f"Part 2: {prod}") 