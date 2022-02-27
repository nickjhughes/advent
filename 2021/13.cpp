#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

using namespace std;

int main() {
    ifstream inputFile;
    inputFile.open("input13");
    if (!inputFile.is_open()) {
        cout << "Failed to open input file." << endl;
        exit(1);
    }
    string line;
    vector<array<int, 2>> points;
    vector<tuple<char, int>> folds;
    while (inputFile) {
        getline(inputFile, line);
        if (line.length() > 0) {
            if (line.substr(0, 4) == "fold") {
                tuple<char, int> fold = make_tuple(
                    line[11], stoi(line.substr(13, line.length() - 13)));
                folds.push_back(fold);
            } else {
                size_t commaPos = line.find(',');
                array<int, 2> point = {
                    stoi(line.substr(0, commaPos)),
                    stoi(line.substr(commaPos + 1,
                                     line.length() - commaPos - 1))};
                points.push_back(point);
            }
        }
    }

    // Part 1
    vector<array<int, 2>> newPoints;
    auto firstFold = folds[0];
    auto axis = get<1>(firstFold);
    if (get<0>(firstFold) == 'x') {
        // Fold left
        for (size_t i = 0; i < points.size(); i++) {
            auto point = points[i];
            array<int, 2> newPoint;
            if (point[0] < axis) {
                newPoint[0] = point[0];
                newPoint[1] = point[1];
            } else {
                newPoint[0] = axis - (point[0] - axis);
                newPoint[1] = point[1];
            }
            if (find(newPoints.begin(), newPoints.end(), newPoint) ==
                newPoints.end()) {
                newPoints.push_back(newPoint);
            }
        }
    } else {
        // Fold up
        for (size_t i = 0; i < points.size(); i++) {
            auto point = points[i];
            array<int, 2> newPoint;
            if (point[1] < axis) {
                newPoint[0] = point[0];
                newPoint[1] = point[1];
            } else {
                newPoint[0] = point[0];
                newPoint[1] = axis - (point[1] - axis);
            }
            if (find(newPoints.begin(), newPoints.end(), newPoint) ==
                newPoints.end()) {
                newPoints.push_back(newPoint);
            }
        }
    }
    cout << newPoints.size() << endl;

    // Part 2
    newPoints.clear();
    for (auto fold : folds) {
        auto axis = get<1>(fold);
        if (get<0>(fold) == 'x') {
            // Fold left
            for (size_t i = 0; i < points.size(); i++) {
                auto point = points[i];
                array<int, 2> newPoint;
                if (point[0] < axis) {
                    newPoint[0] = point[0];
                    newPoint[1] = point[1];
                } else {
                    newPoint[0] = axis - (point[0] - axis);
                    newPoint[1] = point[1];
                }
                if (find(newPoints.begin(), newPoints.end(), newPoint) ==
                    newPoints.end()) {
                    newPoints.push_back(newPoint);
                }
            }
        } else {
            // Fold up
            for (size_t i = 0; i < points.size(); i++) {
                auto point = points[i];
                array<int, 2> newPoint;
                if (point[1] < axis) {
                    newPoint[0] = point[0];
                    newPoint[1] = point[1];
                } else {
                    newPoint[0] = point[0];
                    newPoint[1] = axis - (point[1] - axis);
                }
                if (find(newPoints.begin(), newPoints.end(), newPoint) ==
                    newPoints.end()) {
                    newPoints.push_back(newPoint);
                }
            }
        }
        points = newPoints;
        newPoints.clear();
    }
    int maxX = 0;
    int maxY = 0;
    for (auto point : points) {
        if (point[0] > maxX) {
            maxX = point[0];
        }
        if (point[1] > maxY) {
            maxY = point[1];
        }
    }
    for (int y = 0; y <= maxY; y++) {
        for (int x = 0; x <= maxX; x++) {
            array<int, 2> point = {x, y};
            bool found = false;
            for (auto vectorPoint : points) {
                if (vectorPoint[0] == point[0] && vectorPoint[1] == point[1]) {
                    found = true;
                    break;
                }
            }
            if (found) {
                cout << '#';
            } else {
                cout << '.';
            }
        }
        cout << endl;
    }

    return 0;
}
