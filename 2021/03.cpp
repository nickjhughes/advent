#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::ifstream inputFile;
    inputFile.open("input3");
    if (!inputFile.is_open()) {
        std::cout << "Failed to open input file." << std::endl;
        exit(1);
    }
    std::string line;
    std::vector<std::string> values;
    while (inputFile) {
        std::getline(inputFile, line);
        if (line.length() > 0) {
            values.push_back(line);
        }
    }

    // Part 1

    // std::cout <<  << std::endl;

    // Part 2
    // std::cout <<  << std::endl;

    return 0;
}
