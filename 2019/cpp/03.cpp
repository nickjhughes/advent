#include <algorithm>
#include <iostream>
#include <limits>
#include <string>
#include <unordered_map>
#include <unordered_set>

#include <boost/functional/hash.hpp>

#include "input.h"
#include "utils.h"

using namespace std;

unordered_set<pair<int, int>, boost::hash<pair<int, int>>>
calculate_wire_points(vector<string> moves) {
    int x = 0;
    int y = 0;
    unordered_set<pair<int, int>, boost::hash<pair<int, int>>> points = {
        make_pair(x, y)};
    for (auto move : moves) {
        char direction = move[0];
        int distance = stoi(move.substr(1));
        switch (direction) {
        case 'U': {
            int new_y = y + distance;
            while (y != new_y) {
                points.insert(make_pair(x, ++y));
            }
        } break;
        case 'D': {
            int new_y = y - distance;
            while (y != new_y) {
                points.insert(make_pair(x, --y));
            }
        } break;
        case 'L': {
            int new_x = x - distance;
            while (x != new_x) {
                points.insert(make_pair(--x, y));
            }
        } break;
        case 'R': {
            int new_x = x + distance;
            while (x != new_x) {
                points.insert(make_pair(++x, y));
            }
        } break;
        default:
            cerr << "Unknown direction " << direction << endl;
            exit(1);
            break;
        }
    }
    return points;
}

unordered_map<pair<int, int>, int, boost::hash<pair<int, int>>>
calculate_wire_points_with_steps(vector<string> moves) {
    int x = 0;
    int y = 0;
    int length = 0;
    unordered_map<pair<int, int>, int, boost::hash<pair<int, int>>> points;
    points.emplace(make_pair(x, y), 0);
    for (auto move : moves) {
        char direction = move[0];
        int distance = stoi(move.substr(1));
        switch (direction) {
        case 'U': {
            int new_y = y + distance;
            while (y != new_y) {
                points.emplace(make_pair(x, ++y), ++length);
            }
        } break;
        case 'D': {
            int new_y = y - distance;
            while (y != new_y) {
                points.emplace(make_pair(x, --y), ++length);
            }
        } break;
        case 'L': {
            int new_x = x - distance;
            while (x != new_x) {
                points.emplace(make_pair(--x, y), ++length);
            }
        } break;
        case 'R': {
            int new_x = x + distance;
            while (x != new_x) {
                points.emplace(make_pair(++x, y), ++length);
            }
        } break;
        default:
            cerr << "Unknown direction " << direction << endl;
            exit(1);
            break;
        }
    }
    return points;
}

int main() {
    auto lines = load_input("input03");
    auto wire1 = split_string(lines[0], ",");
    auto wire2 = split_string(lines[1], ",");

    // Part 1
    auto wire1_points = calculate_wire_points(wire1);
    auto wire2_points = calculate_wire_points(wire2);
    int closest_distance = numeric_limits<int>::max();
    for (auto p1 : wire1_points) {
        if (wire2_points.count(p1) > 0) {
            if (!(get<0>(p1) == 0 && get<1>(p1) == 0)) {
                int distance = abs(get<0>(p1)) + abs(get<1>(p1));
                if (distance < closest_distance)
                    closest_distance = distance;
            }
        }
    }
    cout << closest_distance << endl;

    // Part 2
    auto wire1_points_with_steps = calculate_wire_points_with_steps(wire1);
    auto wire2_points_with_steps = calculate_wire_points_with_steps(wire2);
    int smallest_delay = numeric_limits<int>::max();
    for (auto [p1, l1] : wire1_points_with_steps) {
        if (wire2_points_with_steps.count(p1) > 0) {
            if (!(get<0>(p1) == 0 && get<1>(p1) == 0)) {
                int delay = l1 + wire2_points_with_steps.at(p1);
                if (delay < smallest_delay)
                    smallest_delay = delay;
            }
        }
    }
    cout << smallest_delay << endl;

    return 0;
}
