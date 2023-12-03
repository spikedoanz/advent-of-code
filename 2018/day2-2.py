def skip_ind(string, ind):
    return string[:ind] + string[ind+1:]
with open("day2input.txt", 'r') as file:
    for i in range(26):
        seen=set()
        for line in file:
            content=line.strip()
            new = skip_ind(content, i)
            if new in seen:
                print(new)
            else:
                seen.add(new)
        file.seek(0)