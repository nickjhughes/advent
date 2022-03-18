#pragma once

#include <string>
#include <vector>

std::vector<std::string> split_string(std::string str, std::string delimiter) {
    std::vector<std::string> strings;
    size_t last = 0;
    size_t next = 0;
    while ((next = str.find(delimiter, last)) != std::string::npos) {
        strings.push_back(str.substr(last, next - last));
        last = next + 1;
    }
    strings.push_back(str.substr(last));
    return strings;
}
