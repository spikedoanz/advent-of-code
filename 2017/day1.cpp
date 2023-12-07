#include <iostream>
#include <fstream>
#include <string>
#include <sstream>

int main() {
    std::ifstream file("inputday1.txt");
    std::stringstream buffer;

    if (file.is_open()) {
        buffer << file.rdbuf();
        file.close();
    } else {
        std::cout << "Unable to open file";
    }
    std:: string str = buffer.str();
    int start = str[0];
    int last = str[0];
    int curr;
    int tot = 0;
    for (int i=0; i < str.size(); i++) {
        curr = str[i] - 48;
        if (curr == last) {
            tot += curr;
        }
        last = curr;
    }
    if (last == str[0] - 48) {tot += last;};
    printf("Part 1: %d\n", tot);

    int strz = str.size();
    int step = str.size() / 2;
    int tot2 = 0;
    for (int i=0; i < str.size(); i++) {
        curr = str[i] - 48;
        last = str[(i+step) % strz] - 48;
        if (curr == last) {
            tot2 += curr;
        }
    }

    if (last == str[0] - 48) {tot += last;};
    printf("Part 2: %d\n", tot2);
    return 0;
}