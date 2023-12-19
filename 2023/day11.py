lines = [line.strip() for line in open('inputday11.txt').readlines()]
stars = []
def calc_dist(stars, factor):
    ret = 0
    cols = set([x for x in range(len(lines[0]))])
    rows = set([y for y in range(len(lines))])
    for (col, row) in stars:
        if col in cols:
            cols.remove(col)
        if row in rows:
            rows.remove(row)
    for a in stars:
        for b in stars:
            dist = 0
            xdist = list(range(a[0], b[0])) 
            ydist = list(range(a[1], b[1]))
            for x in xdist:
                dist = dist + factor if x in cols else dist + 1
            for y in ydist:
                dist = dist + factor if y in rows else dist + 1
            ret += dist  
    return ret

for y, line in enumerate(lines):
    for x, char in enumerate(line):
        if char == '#':
            stars.append( (x,y) )

print(f"Part 1: {calc_dist(stars, 2)}")
print(f"Part 2: {calc_dist(stars, 1000000)}")

