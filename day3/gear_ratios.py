example = [
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
    ]

def get_nums_with_coords(grid):
    nums_with_coords = []
    for row in range(len(grid)):
        current_num = ""
        current_num_coords = []
        for col in range(len(grid[row])):
            c = grid[row][col]
            if c.isdigit():
                current_num = current_num + c
                current_num_coords.append((row, col))
            else:
                if current_num != "":
                    nums_with_coords.append({'num': int(current_num), 'coords': current_num_coords})
                    current_num = ""
                    current_num_coords = []
        if current_num != "":
            nums_with_coords.append({'num': int(current_num), 'coords': current_num_coords})
    return nums_with_coords

symbols = ['!', '@', '#', '$', '%', '^', '&', '*', '+', '=', '/', '-']

def num_is_part(grid, num_coord) -> bool:
    for (row, col) in num_coord['coords']:
        for delta_x in range(-1, 2):
            for delta_y in range(-1, 2):
                if row + delta_x >= len(grid) or row + delta_x < 0:
                    continue
                if col + delta_y >= len(grid[row]) or col + delta_y < 0:
                    continue
                if grid[row + delta_x][col + delta_y] in symbols:
                    return True
    return False

def part1(grid):
    nums_coords = get_nums_with_coords(grid)
    total = 0
    for num_coord in nums_coords:
        if num_is_part(grid, num_coord):
            total += num_coord['num']
    return total

print(part1(example))

with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(part1(lines))

def gear_map(grid):
    gm = {}
    for row in range(len(grid)):
        for col in range(len(grid[row])):
            if grid[row][col] == '*':
                gm[(row, col)] = []
    return gm

def add_num_to_gears(grid, gm, num_coord):
    for (row, col) in num_coord['coords']:
        for delta_x in range(-1, 2):
            for delta_y in range(-1, 2):
                if row + delta_x >= len(grid) or row + delta_x < 0:
                    continue
                if col + delta_y >= len(grid[row]) or col + delta_y < 0:
                    continue
                if (row + delta_x, col + delta_y) in gm:
                    gm[(row + delta_x, col + delta_y)].append(num_coord['num'])
    return gm

def part2(grid):
    gm = gear_map(grid)
    nums_coords = get_nums_with_coords(grid)
    total = 0
    for num_coord in nums_coords:
        gm = add_num_to_gears(grid, gm, num_coord)
    for v in gm.values():
        vs = set(v)
        if len(vs) == 2:
            total += vs.pop() * vs.pop()
    return total

print(part2(example))

with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(part2(lines))
