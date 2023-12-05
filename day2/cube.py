#!/opt/homebrew/bin/python3

example = [
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ]

def parse_game(line: str):
    id = int(line.split(" ")[1][:-1])
    rounds_str = line.split(":")[1]
    rounds = rounds_str.split(";")
    parsed_rounds = []
    for round in rounds:
        cubes = [cube.strip() for cube in round.split(",")]
        cubes_split = [cube.split(" ") for cube in cubes]
        parsed_rounds.append({cube[1]: int(cube[0]) for cube in cubes_split})

    return {'id': id, 'rounds': parsed_rounds}

# print(parse_game(example[0]))
bag = {
    "red": 12,
    "green": 13,
    "blue": 14
    }

def part1(lines):
    games = [parse_game(line) for line in lines]
    total = 0
    for game in games:
        all_less = True
        for round in game["rounds"]:
            for k, v in round.items():
                if v > bag[k]:
                    all_less = False
        if all_less:
            total += game['id']
    return total

print(part1(example))
with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(part1(lines))

def part2(lines):
    games = [parse_game(line) for line in lines]
    total = 0
    for game in games:
        maximums = {"red": 0, "green": 0, "blue": 0}
        for round in game["rounds"]:
            for k, v in round.items():
                maximums[k] = max(v, maximums[k])
        total += maximums["red"] * maximums["green"] * maximums["blue"]
    return total

print(part2(example))
with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(part2(lines))
