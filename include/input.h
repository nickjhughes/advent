#pragma once

#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> load_input(std::string path) {
    std::ifstream input_file_stream;
    input_file_stream.open(path);
    if (!input_file_stream.is_open()) {
        std::cerr << "Failed to open input file " << path << std::endl;
        exit(1);
    }
    std::vector<std::string> lines;
    std::string line;
    while (input_file_stream) {
        std::getline(input_file_stream, line);
        lines.push_back(line);
    }
    return lines;
}