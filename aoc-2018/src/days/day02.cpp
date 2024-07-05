#include "day02.hpp"
#include <ranges>
#include <set>
#include <utility>
#include <vector>

std::map<char, int> Day02::frequencies(const std::vector<char> &vec) {
  std::map<char, int> freq;
  for (auto c : vec) {
    std::map<char, int>::iterator it = freq.find(c);
    if (it != freq.end()) {
      it->second += 1;
    } else {
      freq.insert(std::make_pair(c, 1));
    }
  }
  return freq;
}

int Day02::count_freq(const std::map<char, int> &freq_map, int target) {
  for (auto item : freq_map) {
    if (item.second == target) {
      return 1;
    }
  }
  return 0;
}

int Day02::part1(const Input &input) {
  int doubles = 0, triples = 0;
  for (auto line : input) {
    auto freq = Day02::frequencies(line);
    doubles += Day02::count_freq(freq, 2);
    triples += Day02::count_freq(freq, 3);
  }
  return doubles * triples;
}

int Day02::part2(const Input &input) {
  const int len = input[0].size();
  for (int idx = 0; idx < len; idx++) {
    std::set<std::string> seen;
    for (auto line : input) {
      std::string staged = std::string(line.begin(), line.end());
      staged.erase(idx, 1);
      if (seen.contains(staged)) {
        std::cout << ":" << staged << ":" << std::endl;
        return 1;
      }
      seen.insert(staged);
    }
  }
  return -1;
}

Input Day02::parse(std::vector<std::string> input) {
  auto result = input | std::views::transform([](std::string s) {
                  return std::vector<char>(s.begin(), s.end());
                });
  return std::vector(result.begin(), result.end());
}
