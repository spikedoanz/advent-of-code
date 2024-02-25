#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>

int evenly_divides(int a, int b) {
    if (a % b == 0) {
        return a / b;
    }
    else {
        return 0;
    }
}

int main() {
    std::ifstream file("inputday2.txt");
    std::string line;
    std::vector<std::string> lines;

    if (file.is_open()) {
        while (std::getline(file, line)) {
            lines.push_back(line);
        }
    } else {
        std::cout << "Unable to open file";
    }

    // Part 1
    int p1 = 0;
    int curr; 
    int diff;
    for (const auto& line : lines) {
        std::istringstream iss(line);
        std::string word;
        int min =  2147483647; 
        int max = -2147483648;
        while (iss >> word) {
            curr = std::stoi(word); 
            if (curr < min) {
                min = curr;
            } 
            if (curr > max) {
                max = curr;
            }
        }
        p1 = p1 + max - min;
    }
    std::cout << "Part 1: " << p1 << std::endl;

    // Part 2
    int p2 = 0;
    int a; int b;
    for (const auto& line : lines) {
        std::istringstream iss(line);
        std::string word;
        std::vector<std::string> words;
        while (iss >> word) { 
            words.push_back(word);
        }
        for (size_t i = 0; i < words.size(); i++) {
            a = stoi(words[i]);
            for (size_t j = 0; j < words.size(); j++) {
                
                if (i == j) {
                    p2 = p2 + 0;
                }
                else {
                    b = stoi(words[j]);
                    p2 = p2 + evenly_divides(a,b);
                    //std::cout << a << " " << b << " " << evenly_divides(a,b) << " i: " << i << " j: " << j << std::endl;
                }
            }
        }
        //std::cout << "\n";
    }
    std::cout << "Part 2: " << p2 << std::endl;
    return 0;
}