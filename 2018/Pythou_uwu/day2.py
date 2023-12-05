with open("inputday2.txt", 'r') as file:
    two, three = 0, 0
    for line in file:
        counts = [line.strip().count(char) for char in 'abcdefghijklmnopqrstuvwxyz']
        two = two + 1 if 2 in counts else two
        three = three + 1 if 3 in counts else three

print(f'Part 1: {two*three}')

with open("inputday2.txt", 'r') as file:
    for i in range(26):
        seen=set()
        for line in file:
            content=line.strip()
            new = content[:i] + content[i+1:]
            if new in seen:
                print(f'Part 2: {new}')
            else:
                seen.add(new)
        file.seek(0)