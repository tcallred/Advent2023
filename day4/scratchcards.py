example = [
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
    "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
    "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
    "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
    "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ]

def parse(lines):
    cards = []
    for line in lines:
        rhs = line.split(":")[1]
        numlists = [numlist.strip() for numlist in rhs.split("|")]
        cards.append([[int(num) for num in numlist.split()] for numlist in numlists])
    return cards

def points(n):
    x = n - 1
    if x < 0:
        return 0
    return 2**x

def part1(lines):
    return sum([points(len(set(card[0]).intersection(set(card[1])))) for card in parse(lines)])

print(part1(example))
    
with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(part1(lines))

def part2(lines):
    points = [len(set(card[0]).intersection(set(card[1]))) for card in parse(lines)]
    cardcount = {idx: 1 for idx in range(len(points))}
    for idx in range(len(points)):
        p = points[idx]
        for _ in range(cardcount[idx]):
            for x in range(idx + 1, idx + p + 1):
                cardcount[x] += 1
    return sum(cardcount.values())

print(part2(example))

with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(part2(lines))
