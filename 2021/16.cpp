#include <bitset>
#include <fstream>
#include <iostream>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

enum PacketType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo
};

struct Packet {
    uint8_t version;
    PacketType type;
    vector<Packet *> subPackets;
} tPacket;

void parsePackets(const vector<bool> *bits, size_t *bitsRead) {
    vector<Packet> packets;
    size_t i = 0;
    if (accumulate(bits->begin(), bits->end(), 0) == 0) {
        bitsRead = 0;
        return;
    }
    Packet packet;
    bitset<3> version(vector);
    // packet.version = bits[.substr(0, 3)];
    // packet.type = PacketType?
}

int main() {
    ifstream inputFile;
    inputFile.open("input16");
    if (!inputFile.is_open()) {
        cout << "Failed to open input file." << endl;
        exit(1);
    }
    string hexString;
    getline(inputFile, hexString);
    hexString = "D2FE28";

    // Convert hex string to vector<bool>
    vector<bool> bits;
    bits.reserve(hexString.length() * 4);
    for (size_t i = 0; i < hexString.length(); i++) {
        uint8_t n = strtol(hexString.substr(i, 1).c_str(), NULL, 16);
        bits.insert(bits.end(),
                    {(bool)((n & 0b1000) >> 3), (bool)((n & 0b0100) >> 2),
                     (bool)((n & 0b0010) >> 1), (bool)(n & 0b0001)});
    }

    size_t bitsRead;
    parsePackets(&bits, &bitsRead);

    return 0;
}
