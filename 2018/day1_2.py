def main():
    ret = 0
    seen = set()
    with open("day1input.txt", 'r') as file:
        found = False 
        while (not found):
            for line in file:
                content = line.strip()
                if content[0] == '+':
                    ret += int(content[1:]) 
                else:
                    ret -= int(content[1:])
                if ret in seen:
                    print(f"ding ding ding {ret}")
                    found = True 
                    return(ret)
                else:
                    seen.add(ret)
            file.seek(0)
            print(len(seen))
        
print(main())