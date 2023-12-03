ret = 0
with open("day1input.txt", 'r') as file:
    for line in file:
        content = line.strip()
        if content[0] == '+':
            ret += int(content[1:]) 
        else:
            ret -= int(content[1:])
print(ret)