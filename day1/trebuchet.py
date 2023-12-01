# ----- Part 1 ------

test_input = [
    "1abc2",
    "pqr3stu8vwx",
    "a1b2c3d4e5f",
    "treb7uchet"
    ]

def digits(s: str) -> str:
    return "".join(([c for c in s if c.isdigit()]))

def first_and_last_digits(s: str) -> int:
    first = s[0]
    last = s[len(s) - 1]
    return int(first + last)

def sum_calibration(calibration_vals: list[str]) -> int:
    return sum([first_and_last_digits(digits(val)) for val in calibration_vals])

print(sum_calibration(test_input))

with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(sum_calibration(lines))

# ----- Part 2 ------

test_input_2 = [
    "two1nine",
    "eightwothree",
    "abcone2threexyz",
    "xtwone3four",
    "4nineeightseven2",
    "zoneight234",
    "7pqrstsixteen"
    ]

number_map = {
    "one": "1",
    "two": "2" ,       
    "three":"3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

def calibration_to_number(s: str) -> int:
    all_nums = [*number_map.keys(), *number_map.values()] 
    matches = []
    for num in all_nums:
        pos = s.find(num)
        while pos != -1:
            if num.isalpha():
                matches.append((number_map[num], pos))
            else:
                matches.append((num, pos))
            pos = s.find(num, pos + 1)
    matches.sort(key=lambda x: x[1])
    return int(matches[0][0] + matches[-1][0])

def sum_calibration_2(calibrations):
    return sum([calibration_to_number(cal) for cal in calibrations])

print(sum_calibration_2(test_input_2))

with open("input.txt", 'r') as file:
    lines = file.readlines()
    print(sum_calibration_2(lines))



