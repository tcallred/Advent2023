#include <cctype>
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

static std::string ex1 = "1abc2";

int first_and_last_digits(const std::string& str) {
    char first = 0;
    char last = 0;
    for(char ch : str) {
        if (std::isdigit(ch)) {
            last = ch;
            if (first == 0) {
                first = ch;
            }
        }
    }
    std::string dstr = {first, last};
    return std::stoi(dstr);
}

static const std::unordered_map<std::string, char> word_to_digit = {
    {"one", '1'}, {"two", '2'}, {"three", '3'}, {"four", '4'},
    {"five", '5'}, {"six", '6'}, {"seven", '7'}, {"eight", '8'}, {"nine", '9'}
};

char digit_from_word(const std::string& word) {
    auto it = word_to_digit.find(word);
    if (it != word_to_digit.end()) {
        return it->second;
    }
    return 0;
}

int part2(const std::string& str) {
    char first = 0;
    char last = 0;
    for (size_t i = 0; i < str.length(); ++i) {
        if (std::isdigit(str[i])) {
            last = str[i];
            if (first == 0) {
                first = str[i];
            }
        } else {
            for (const auto& [word, digit] : word_to_digit) {
                if (str.compare(i, word.length(), word) == 0) {
                    last = digit;
                    if (first == 0) {
                        first = digit;
                    }
                    i += word.length() - 1;
                    break;
                }
            }
        }
    }
    std::string dstr = {first, last};
    return std::stoi(dstr);
}

int main() {
    int ans = first_and_last_digits(ex1);
    std::cout << ans << std::endl;
    int ans2 = part2("7pqrstsixteen");
    std::cout << ans2 << std::endl;
}
