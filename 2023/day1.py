calibrate=0
with open("inputday1.txt", 'r') as file:
    for line in file:
        numbers = [int(x) for x in line.strip() if x in '0123456789']
        calibrate+=numbers[0]*10 + numbers[-1] 

print(f'Part 1: {calibrate}')

calibrate=0
written=['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
with open("inputday1.txt", 'r') as file:
    for line in file:
        numbers = []
        content=line.strip()
        for i, _ in enumerate(content):
            if _ in '0123456789':
                numbers.append(int(_)) 
            else:
                for j, name in enumerate(written):
                    if content[i:].startswith(name):
                        numbers.append(j)
        calibrate+=numbers[0]*10 + numbers[-1] 

print(f'Part 2: {calibrate}')
