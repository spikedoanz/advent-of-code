def process_part1(curr, record):

    def find_dest(curr, source, record):
        right = source + record[source][1]
        if curr < right: 
            return record[source][0] + curr - source
        else:
            return curr

    sources = record['sources']
    if curr < sources[0]:
        return curr
    for i, source in enumerate(sources):
        if source <= curr and (i+1) < len(sources) and sources[i+1] > curr:
            return find_dest(curr, source, record)
    return find_dest(curr, sources[-1], record)

def process_part2(seed, src, mpp):

    def intersection(a, b, c, d):  
        start = max(a,c)
        end = min(b,d)
        if start > end:
            return [] 
        else:
            return [(start, end)]

    curr_seed = []
    temp = seed[0] 
    for i, _ in enumerate(src):
        uni = intersection(*seed, *_)
        if uni != []:
            if temp < uni[0][0]:
                curr_seed.append( (temp, uni[0][0]-1))
            mapped = (uni[0][0] + mpp[i], uni[0][1] + mpp[i]) 
            curr_seed.append(mapped)
            seed = (uni[0][1]+1, seed[1]) if uni[0][1]+1 < seed[1] else seed
            temp = uni[0][1]+1
    return curr_seed

def create_table(lines):
    table = [] 
    curr = {'sources': []}
    for i, line in enumerate(lines[2:]):
        content=line.strip()
        if (content[-4:] == 'map:'): 
            curr = {'sources':[]} 
        elif (content == ""):
            curr['sources'].sort()
            table.append(curr)
        else:
            content = [int(x) for x in content.split()]
            curr['sources'].append(content[1])
            content = {content[1]: (content[0], content[2])} # Source : Destination, Range
            curr = {**curr, **content}
    curr['sources'].sort()
    table.append(curr)
    return table

def part_1(seeds, table):
    p1 = []
    for curr in seeds:
        for record in table:
            curr = process_part1(curr, record)
        p1.append(curr) 
    return min(p1)

def part_2(seeds, table):
    seeds2 = [] # Create secondary seeds table that match part 2's specs
    for i, seed in enumerate(seeds):
        start = seed if i % 2 == 0 else start 
        if i % 2 == 1:
            end = seed + start - 1
            seeds2.append((start, end))
    seeds2 = sorted(seeds2)

    table_src, table_mpp = [], [] # Create intermediate tables
    for i, _ in enumerate(table): 
        sources = _['sources']
        srcs, mpps = [], []
        for source in sources:
            src = (source, source + _[source][1] - 1)
            mpp = (_[source][0] - source)
            srcs.append(src)
            mpps.append(mpp)
        table_src.append(srcs)
        table_mpp.append(mpps)

    for i in range(len(table_mpp)): 
        new_layer = []
        for seed in seeds2:
            new_layer.extend(process_part2(seed, table_src[i], table_mpp[i]))
        seeds2 = new_layer
    return min([x[0] for x in seeds2])
 
if __name__=="__main__":
    fname = "inputday5.txt"
    with open(fname, 'r') as f:
        lines = f.readlines()
    seeds = [int(x) for x in lines[0].strip().split(':')[1].split()]
    table = create_table(lines)

    print(f"Part 1: {part_1(seeds, table)}")
    print(f"Part 2: {part_2(seeds, table)}")