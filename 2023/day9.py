def zeroes(arr):
    return all(x == 0 for x in arr) 
def part1(lines):
    p1 ,pred = 0, 0
    for line in lines:
        curr = [int(x) for x in line.strip().split()]
        while(not zeroes(curr)):
            pred += curr[-1]
            temp = []
            for j in range(len(curr)-1):
                temp.append(curr[j+1]-curr[j]) 
            curr = temp
        p1 += pred
        pred = 0
    return p1

def part2(lines):
    p2, pred = 0, 0
    for line in lines:
        curr = [int(x) for x in line.strip().split()]
        pred = curr[0]
        odd = -1 # left prediction flip flops through addition and subtraction
        while(not zeroes(curr)):
            temp = []
            for j in range(len(curr)-1):
                temp.append(curr[j+1]-curr[j]) 
            curr = temp
            pred = pred - curr[0] if odd == -1 else pred + curr[0]
            odd *= -1
        p2 += pred
        pred = 0
    return p2

if __name__ == "__main__":
    lines = open('inputday9.txt').readlines()
    print(f'Part 1: {part1(lines)}')
    print(f'Part 2: {part2(lines)}')
