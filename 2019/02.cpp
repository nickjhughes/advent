#include <iostream>
#include <string>

#include "input.h"
#include "utils.h"

using namespace std;

enum class OpCode {
    ADDITION = 1,
    MULTIPLICATION = 2,
    HALT = 99,
};

void print_integers(vector<int> integers) {
    for (size_t i = 0; i < integers.size() - 1; ++i)
        cout << integers[i] << ",";
    cout << integers[integers.size() - 1] << endl;
}

void run_program(vector<int> &integers) {
    int i = 0;
    bool halted = false;
    while (!halted) {
        auto op_code = static_cast<OpCode>(integers[i]);
        switch (op_code) {
        case OpCode::ADDITION: {
            int i1 = integers[i + 1];
            int i2 = integers[i + 2];
            int i3 = integers[i + 3];
            integers[i3] = integers[i1] + integers[i2];
            i += 4;
        } break;
        case OpCode::MULTIPLICATION: {
            int i1 = integers[i + 1];
            int i2 = integers[i + 2];
            int i3 = integers[i + 3];
            integers[i3] = integers[i1] * integers[i2];
            i += 4;
        } break;
        case OpCode::HALT: {
            halted = true;
        } break;
        default:
            cerr << "Unexpected opcode: " << integers[i] << endl;
            exit(1);
        }
    }
}

int main() {
    auto lines = load_input("input02");
    auto strings = split_string(lines[0], ",");
    vector<int> integers;
    for (auto s : strings)
        integers.push_back(stoi(s));

    // Part 1
    vector<int> part1;
    for (auto i : integers)
        part1.push_back(i);
    part1[1] = 12;
    part1[2] = 2;
    run_program(part1);
    cout << part1[0] << endl;

    // Part 2
    for (int a = 0; a <= 99; ++a) {
        for (int b = 0; b <= 99; ++b) {
            vector<int> part2;
            for (auto i : integers)
                part2.push_back(i);
            part2[1] = a;
            part2[2] = b;
            run_program(part2);
            if (part2[0] == 19690720) {
                cout << 100 * a + b << endl;
                goto end;
            }
        }
    }

end:
    return 0;
}
