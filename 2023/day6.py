time =  [54, 94, 65, 92]
dist = [302, 1476, 1029, 1404]

p1= 1
for i, _ in enumerate(time):
    count = 0
    for j in range(_):
        distcurr= (_-j) * (j)
        if distcurr > dist[i]:
            count += 1
    p1 *= count 
print(f"Part 1: {p1}")

time2 = 54946592
dist2 = 302147610291404
count = 0
p2 =1 
for j in range(time2):
    distcurr= (time2-j) * (j)
    if distcurr > dist2:
        count += 1

print(f"Part2: {p2}")

