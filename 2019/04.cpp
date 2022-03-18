#include <iostream>
#include <string>

#include "input.h"
#include "utils.h"

using namespace std;

bool validate_password(int password) {
    string str = to_string(password);

    // Critera 1: Two adjacent digits are the same (like 22 in 122345).
    bool has_double = false;
    for (size_t i = 0; i < str.length() - 1; ++i) {
        if (str[i] == str[i + 1])
            has_double = true;
    }

    bool non_decreasing = true;
    // Critera 2: Going from left to right, the digits never decrease; they only
    //            ever increase or stay the same (like 111123 or 135679).
    for (size_t i = 0; i < str.length() - 1; ++i) {
        if (str[i + 1] < str[i])
            non_decreasing = false;
    }

    return has_double && non_decreasing;
}

bool validate_password_2(int password) {
    string str = to_string(password);

    // Critera 1: Two adjacent digits are the same (like 22 in 122345).
    // The two adjacent matching digits are not part of a larger group of
    // matching digits.
    bool has_double = false;
    for (size_t i = 0; i < str.length() - 1; ++i) {
        if (str[i] == str[i + 1]) {
            if (i > 0) {
                if (str[i - 1] == str[i]) {
                    continue;
                }
            }
            if (i < str.length() - 2) {
                if (str[i] == str[i + 2]) {
                    continue;
                }
            }
            has_double = true;
        }
    }

    bool non_decreasing = true;
    // Critera 2: Going from left to right, the digits never decrease; they only
    //            ever increase or stay the same (like 111123 or 135679).
    for (size_t i = 0; i < str.length() - 1; ++i) {
        if (str[i + 1] < str[i])
            non_decreasing = false;
    }

    return has_double && non_decreasing;
}

int main() {
    auto lines = load_input("input04");
    auto endpoints = split_string(lines[0], "-");
    int range_start = stoi(endpoints[0]);
    int range_end = stoi(endpoints[1]);

    // Part 1
    int valid_count = 0;
    for (int password = range_start; password <= range_end; ++password) {
        if (validate_password(password))
            ++valid_count;
    }
    cout << valid_count << endl;

    // Part 2
    valid_count = 0;
    for (int password = range_start; password <= range_end; ++password) {
        if (validate_password_2(password))
            ++valid_count;
    }
    cout << valid_count << endl;

    return 0;
}
