#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::ifstream inputFile;
    inputFile.open("input2");
    if (!inputFile.is_open()) {
        std::cout << "Failed to open input file." << std::endl;
        exit(1);
    }
    std::string line;
    std::vector<std::string> directions;
    std::vector<std::uint8_t> values;
    while (inputFile) {
        std::getline(inputFile, line);
        if (line.length() > 0) {
            std::size_t spacePos = line.find(' ');
            directions.push_back(line.substr(0, spacePos));
            values.push_back(
                stoi(line.substr(spacePos, line.length() - spacePos)));
        }
    }

    // Part 1
    uint32_t horz = 0;
    int32_t depth = 0;
    for (int i = 0; i < directions.size(); i++) {
        if (directions[i] == "forward") {
            horz += values[i];
        } else if (directions[i] == "down") {
            depth += values[i];
        } else if (directions[i] == "up") {
            depth -= values[i];
        }
    }
    std::cout << horz * depth << std::endl;

    // Part 2
    horz = 0;
    depth = 0;
    int32_t aim = 0;
    for (int i = 0; i < directions.size(); i++) {
        if (directions[i] == "forward") {
            horz += values[i];
            depth += aim * values[i];
        } else if (directions[i] == "down") {
            aim += values[i];
        } else if (directions[i] == "up") {
            aim -= values[i];
        }
    }
    std::cout << horz * depth << std::endl;

    return 0;
}
