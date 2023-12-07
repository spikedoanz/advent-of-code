def parse_hand(line):
    hand = line.split()[0] 
    cards = set(hand)
    if len(cards) == 1:
        print("five of a kind")
        return 0
    if len(cards) == 2:
        check = hand[0]
        if hand.count(check) == 4:
            print("four of a kind")
            return 1
        print("full house")
        return 2
    if len(cards) == 3:
        for _ in hand:
            if hand.count(_) == 3:
                print("three of a kind")
                return 3
        print("two pair")
        return 4
    if len(cards) == 4:
        print("one pair")
        return 5
    print("high card")
    return 6


lines = open("./inputday7.txt").readlines()
for line in lines[:10]:
    print(line.strip()) 
    parse_hand(line.strip())



