#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <set>


bool no_repeats(std::string s) {
    std::set<std::string> words;
    std::string token;
    std::istringstream tokenStream(s);
    while(std::getline(tokenStream, token, ' ')) {
        if (words.find(token) != words.end()) {
            return false;
        }
        else {
            words.insert(token);
        }
    }        
    return true;
}


bool is_anagram(std::string a, std::string b) {
    if (a.size() != b.size()) {
        return false; 
    }
    std::vector<int> achars(26,0); 
    std::vector<int> bchars(26,0); 
    for (int i = 0; i < a.size(); i++) {
        achars[a[i]-'a']++;
        bchars[b[i]-'a']++;
    }
    for (int i = 0; i < achars.size(); i++) {
        if (achars[i] - bchars[i] != 0) {
            return false;
        }
    }
    return true;
}
        

bool no_anagrams(std::string s) {
    std::vector<std::string> words;
    std::string token;
    std::istringstream tokenStream(s);
    while(std::getline(tokenStream, token, ' ')) {
        words.push_back(token);
    }        
    
    for (int i = 0; i < words.size(); i++) {
        for (int j = i+1; j < words.size(); j++) {
            if (is_anagram(words[i], words[j])) {
                return false;
            }
        }
    }
    return true;


}
int main() {
    std::ifstream file("inputday4.txt");
    std::string line;
    std::vector<std::string> lines;
    while (std::getline(file, line)) {lines.push_back(line);}

    int p1 = 0;
    for (int i = 0; i < lines.size(); i++) {
        if (no_repeats(lines[i])) {
            p1++;
        }
    }
    std::cout << "Part 1: " << p1 << std::endl;

    int p2 = 0;
    for (int i = 0; i < lines.size(); i++) {
        if (no_anagrams(lines[i])) {
            p2++;
        }
    }
    std::cout << "Part 2: " << p2 << std::endl;
    return 0;
}