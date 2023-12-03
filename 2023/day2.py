info=[]
valid=[]

with open("day2.txt", "r") as file:
    index=1
    for line in file:
        content=line.split()
        info.append({
            'index': index,
            'red': 0,
            'blue': 0,
            'green': 0,
        })
        useful = content[2:]
        for i, _ in enumerate(useful):
            if i % 2 == 1:
                extra = int(useful[i-1])
                if 'red' in _:
                    info[index-1]['red'] = extra if  extra > info[index-1]['red'] else info[index-1]['red']
                elif 'blue' in _:
                    info[index-1]['blue'] = extra if  extra > info[index-1]['blue'] else info[index-1]['blue']
                elif 'green' in _:
                    info[index-1]['green'] = extra if  extra > info[index-1]['green'] else info[index-1]['green']
        # 12 red, 13 green and 14 blue 
        if info[index-1]['red'] <= 12 and info[index-1]['green'] <= 13 and info[index-1]['blue'] <= 14:
            valid.append(index)
        index+=1

print(f'Part 1: {sum(valid)}')
total=0
for _ in info:
    total+=_['red']*_['green']*_['blue']
print(f'Part 2: {total}')


