#include <fstream>
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::ifstream inputFile;
    inputFile.open("input1");
    if (!inputFile.is_open()) {
        std::cout << "Failed to open input file." << std::endl;
        exit(1);
    }
    std::string line;
    std::vector<uint32_t> numbers;
    while (inputFile) {
        std::getline(inputFile, line);
        if (line.length() > 0) {
            numbers.push_back(stoi(line));
        }
    }

    // Part 1
    int increaseCount = 0;
    for (int i = 1; i < numbers.size(); i++) {
        if (numbers[i] > numbers[i - 1]) {
            increaseCount++;
        }
    }
    std::cout << increaseCount << std::endl;

    // Part 2
    std::vector<uint32_t> rollingSum;
    for (int i = 0; i < numbers.size() - 2; i++) {
        rollingSum.push_back(numbers[i] + numbers[i + 1] + numbers[i + 2]);
    }
    increaseCount = 0;
    for (int i = 1; i < rollingSum.size(); i++) {
        if (rollingSum[i] > rollingSum[i - 1]) {
            increaseCount++;
        }
    }
    std::cout << increaseCount << std::endl;

    return 0;
}
