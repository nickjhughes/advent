#include <iostream>
#include <string>

#include "input.h"
#include "utils.h"

using namespace std;

int main() {
    auto lines = load_input("input19");
    auto instructions = split_string(lines[0], ",");
    for (auto i : instructions) {
        cout << i << endl;
    }

    // Part 1

    // Part 2

    return 0;
}
