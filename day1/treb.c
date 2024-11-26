#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char* ex1 = "1abc2";

int first_and_last_digits(char* str) {
    char first = 0;
    char last = 0;
    for(int i = 0; str[i] != '\0'; i++) {
        if (isdigit(str[i])) {
            last = str[i];
            if (first == 0) {
                first = str[i];
            }
        }
    }
    char dstr[2] = {first, last};
    return atoi(dstr);
}

char digit_from_word(char* word) {
    if (strcmp(word, "one") == 0) {
        return '1';
    }
    if (strcmp(word, "two") == 0) {
        return '2';
    }
    if (strcmp(word, "three") == 0) {
        return '3';
    }
    if (strcmp(word, "four") == 0) {
        return '4';
    }
    if (strcmp(word, "five") == 0) {
        return '5';
    }
    if (strcmp(word, "six") == 0) {
        return '6';
    }
    if (strcmp(word, "seven") == 0) {
        return '7';
    }
    if (strcmp(word, "eight") == 0) {
        return '8';
    }
    if (strcmp(word, "nine") == 0) {
        return '9';
    }
    return 0;
}

static char* num_words[] = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

int part2(char* str) {
    char first = 0;
    char last = 0;
    for(int i = 0; str[i] != '\0'; i++) {
        if (isdigit(str[i])) {
            last = str[i];
            if (first == 0) {
                first = str[i];
            }
        } else {
            for (int j = 0; j < 10; j++) {
                if (strncmp(str + i, num_words[j], strlen(num_words[j])) == 0) {
                    last = digit_from_word(num_words[j]);
                    if (first == 0) {
                        first = digit_from_word(num_words[j]);
                    }
                    i += strlen(num_words[j]);
                }
            }
        }
    }
    char dstr[2] = {first, last};
    return atoi(dstr);
}

int main() {
    int ans = first_and_last_digits(ex1);
    printf("%d\n", ans);
    int ans2 = part2("7pqrstsixteen");
    printf("%d\n", ans2);
}