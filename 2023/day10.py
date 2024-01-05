def get_links(char, x,y):
    possible = ((x, y-1), (x, y+1), (x-1, y), (x+1, y)) # up down left right 
    links = {
        '|' : (possible[0], possible[1]),
        'J' : (possible[0], possible[2]),
        'L' : (possible[0], possible[3]),
        '7' : (possible[1], possible[2]),
        'F' : (possible[1], possible[3]),
        '-' : (possible[2], possible[3]),
        'S' : possible 
    }
    return links[char]

def dfs(x, y, lines):
    seen = set()
    length = 1
    seen.add((x,y))
    stack = [_ for _ in get_links(lines[y][x], x,y) if _ not in seen]
    while stack:
        (nx, ny) = stack.pop()  
        if nx < len(lines[0]) and ny < len(lines) and nx >= 0 and ny >= 0:
            if lines[ny][nx] != '.': 
                if (nx, ny) not in seen :
                    seen.add((nx, ny))
                    link = [_ for _ in get_links(lines[ny][nx], nx,ny) if _ not in seen]
                    stack.extend(link)
                    length += 1
    for (x,y) in seen:
        nline = lines[y][:x] + '0' + lines[y][x+1:]
        lines[y] = nline
    with open('inputday10.out', 'w') as f:
        for line in lines:
            f.write("%s\n" % line)
    return length 

def interior(lines):
    inn = -1 
    ret = 0
    for line in lines:
        for char in line:
            if 
    return ret  

if __name__ == '__main__':
    lines = [line.strip() for line in open('inputday10.test').readlines()]
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char == 'S':
                print(f'Part 1: {dfs(x, y, lines)//2}')
    lines2 = [line.strip() for line in open('inputday10.out').readlines()]
    print(f'Part 2: {interior(lines2)}')

    