ret = 0
with open("inputday1.txt", 'r') as file:
    for line in file:
        content = line.strip()
        if content[0] == '+':
            ret += int(content[1:]) 
        else:
            ret -= int(content[1:])
print(f'Part 1: {ret}')

ret = 0
seen = set()
with open("inputday1.txt", 'r') as file:
    found = False 
    while (not found):
        for line in file:
            content = line.strip()
            if content[0] == '+':
                ret += int(content[1:]) 
            else:
                ret -= int(content[1:])
            if ret in seen:
                found = True
                break
            else:
                seen.add(ret)
        file.seek(0)
    
print(f'Part 2: {ret}')