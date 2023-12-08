def parse(filename):
    lines = open(filename).readlines()
    inst = lines[0].strip()
    jump = {}
    a = []
    for line in lines[2:]:
        line = line.strip().split(" = ")
        src = line[0]
        if src[-1] == 'A': a.append(src)
        dst = line[1].split(", ")
        lft, rgt = dst[0][1:], dst[1][:-1]
        jump[src] = (lft, rgt)
    return  jump, inst, a

def part1(curr, jump, inst):
    p1=0
    while (curr != 'ZZZ'):
        for _ in inst:
            curr = jump[curr][1] if _ == 'R' else jump[curr][0]
            p1 += 1
    return p1

def part2(a, jump, inst):

    def LCM(ns):
        def factorize(n):
            factors = set() 
            for _ in range(n)[2:]:
                if n % _ == 0:
                    factors.add(_)
            return factors
        
        factors = set()
        for n in ns:
            factors = factors | factorize(n)
        ret = 1
        for _ in factors:
            ret *= _
        return ret

    p2 = []
    for curr in a:
        ret=0
        while (curr[-1] != 'Z'):
            for _ in inst:
                curr = jump[curr][1] if _ == 'R' else jump[curr][0]
                ret += 1
        p2.append(ret)
    return LCM(p2) 

if __name__ == "__main__":
    jump, inst, a = parse('inputday8.txt')
    print(f"Part 1: {part1('AAA', jump, inst)}")
    print(f"Part 2: {part2(a, jump, inst)}")