with open("day2input.txt", 'r') as file:
    two = 0 
    three = 0
    for line in file:
        print(line.strip())
        counts = [line.strip().count(char) for char in 'abcdefghijklmnopqrstuvwxyz']
        if 2 in counts:
            two+=1
        if 3 in counts:
            three+=1
    print(two, three)
    print(two*three)