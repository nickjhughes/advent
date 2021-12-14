#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <string>

using namespace std;

int main() {
    ifstream inputFile;
    inputFile.open("input14");
    if (!inputFile.is_open()) {
        cout << "Failed to open input file." << endl;
        exit(1);
    }
    string line;
    string polymer;
    map<pair<char, char>, char> rules;
    getline(inputFile, polymer);
    while (inputFile) {
        getline(inputFile, line);
        if (line.length() > 0) {
            rules.insert({{line[0], line[1]}, line[6]});
        }
    }

    // Parts 1 & 2
    char firstChar = polymer.front();
    char lastChar = polymer.back();
    map<pair<char, char>, uint64_t> pairCounts;
    for (size_t i = 0; i < polymer.length() - 1; i++) {
        pair<char, char> pair = {polymer[i], polymer[i + 1]};
        if (pairCounts.count(pair) == 0) {
            pairCounts.insert({pair, 0});
        }
        pairCounts[pair] += 1;
    }
    for (uint8_t step = 0; step < 40; step++) {
        map<pair<char, char>, uint64_t> newPairCounts;
        for (auto &pairCount : pairCounts) {
            auto charPair = pairCount.first;
            pair<char, char> newPair1 = {charPair.first, rules[charPair]};
            pair<char, char> newPair2 = {rules[charPair], charPair.second};
            if (newPairCounts.count(newPair1) == 0) {
                newPairCounts.insert({newPair1, 0});
            }
            if (newPairCounts.count(newPair2) == 0) {
                newPairCounts.insert({newPair2, 0});
            }
            newPairCounts[newPair1] += pairCount.second;
            newPairCounts[newPair2] += pairCount.second;
        }
        pairCounts = newPairCounts;

        if (step == 9 || step == 39) {
            map<char, uint64_t> charCounts;
            charCounts.insert({firstChar, 1});
            charCounts.insert({lastChar, 1});
            for (auto &pairCount : pairCounts) {
                if (charCounts.count(pairCount.first.first) == 0) {
                    charCounts.insert({pairCount.first.first, 0});
                }
                if (charCounts.count(pairCount.first.second) == 0) {
                    charCounts.insert({pairCount.first.second, 0});
                }
                charCounts[pairCount.first.first] += pairCount.second;
                charCounts[pairCount.first.second] += pairCount.second;
            }
            if (step == 9) {
                cout << "Part 1" << endl;
            } else if (step == 39) {
                cout << "Part 2" << endl;
            }
            uint64_t maxCount = 0;
            uint64_t minCount = numeric_limits<uint64_t>::max();
            for (auto &charCount : charCounts) {
                if (charCount.second / 2 > maxCount) {
                    maxCount = charCount.second / 2;
                }
                if (charCount.second / 2 < minCount) {
                    minCount = charCount.second / 2;
                }
            }
            cout << maxCount - minCount << endl;
        }
    }

    return 0;
}
