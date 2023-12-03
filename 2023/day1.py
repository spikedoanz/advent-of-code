calibrate=0
with open("day1.txt", 'r') as file:
    for line in file:
        content=line.strip()
        numbers = [int(x) for x in content if x in '0123456789']
        first=numbers[0]
        last=numbers[-1]
        curr=first*10 + last
        calibrate+=curr
print(f'Part 1: {calibrate}')

calibrate=0
written=['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
with open("day1.txt", 'r') as file:
    for line in file:
        numbers = []
        content=line.strip()
        # Parse numbers
        for i, _ in enumerate(content):
            if _ in '0123456789':
                numbers.append(int(_)) 
            else:
                for j, name in enumerate(written):
                    if content[i:].startswith(name):
                        numbers.append(j)
        first=numbers[0]
        last=numbers[-1]
        curr=first*10 + last
        calibrate+=curr
print(f'Part 2: {calibrate}')
