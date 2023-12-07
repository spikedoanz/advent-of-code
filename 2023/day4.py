tot = 0
fname = 'inputday4.test'
with open(fname, 'r') as f:
    lines = f.readlines()
    cards = [[0] for _ in range(len(lines))]
    for i,line in enumerate(lines):
        useful = line.strip().split(':')[1]
        win, hav = useful.split('|')
        hit = set(win.split()) & set(hav.split())
        tot = tot +  2**(len(hit)-1) if len(hit) != 0 else tot 
        cards[i][0] = len(hit) 

print(f'Part 1: {tot}')

ccount = 0
for i, _ in enumerate(cards):
    ccount += len(_)
    for point in _:
        for j in range(point):
            if i+j < len(cards):
                cards[i+j+1].append(cards[i+j+1][0])

print(f'Part 2: {ccount}')

