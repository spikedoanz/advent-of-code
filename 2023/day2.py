info=[]
valid=[]

with open("inputday2.txt", "r") as file:
    for index, line in enumerate(file):
        info.append({'index': index,'red': 0,'blue': 0,'green': 0,})
        useful = line.split()[2:]
        for i, _ in enumerate(useful):
            if i % 2 == 1:
                extra = int(useful[i-1])
                for color in ['red', 'green', 'blue']:
                    info[index-1][color] = extra if extra > info[index-1][color] and color in _ else info[index-1][color]
        if info[index-1]['red'] <= 12 and info[index-1]['green'] <= 13 and info[index-1]['blue'] <= 14:
            valid.append(index)

print(f'Part 1: {sum(valid)}')

total=0
for _ in info:
    total+=_['red']*_['green']*_['blue']
print(f'Part 2: {total}')


