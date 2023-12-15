#include <iostream>
#include <math.h>
#include <string>

#include "input.h"

using namespace std;

int calculate_fuel_from_mass(int mass) { return floor(mass / 3.0f) - 2; }

int calculate_total_fuel_from_mass(int mass) {
    int base_fuel = calculate_fuel_from_mass(mass);
    int total_fuel = 0;
    int extra_fuel = base_fuel;
    do {
        total_fuel += extra_fuel;
        extra_fuel = calculate_fuel_from_mass(extra_fuel);
    } while (extra_fuel >= 0);
    return total_fuel;
}

int main() {
    auto lines = load_input("input01");

    // Part 1
    int total_fuel = 0;
    for (auto line : lines) {
        if (line.size() == 0)
            continue;
        int mass = stoi(line);
        total_fuel += calculate_fuel_from_mass(mass);
    }
    cout << total_fuel << endl;

    // Part 2
    int true_total_fuel = 0;
    for (auto line : lines) {
        if (line.size() == 0)
            continue;
        int mass = stoi(line);
        int module_fuel = calculate_total_fuel_from_mass(mass);
        true_total_fuel += calculate_total_fuel_from_mass(mass);
    }
    cout << true_total_fuel << endl;

    return 0;
}
