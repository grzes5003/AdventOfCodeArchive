#include "day01.hpp"
#include <numeric>
#include <ranges>
#include <set>

int Day01::part1(const std::vector<int> &input) {
  return std::accumulate(input.begin(), input.end(), 0);
}

int Day01::part2(const std::vector<int> &input) {
  std::set<int> seen;
  int freq = 0;
  while (true) {
    for (auto item : input) {
      freq += item;
      if (seen.contains(freq)) {
        return freq;
      }
      seen.insert(freq);
    }
  }
}

std::vector<int> Day01::parse(std::vector<std::string> input) {
  auto result =
      input | std::views::transform([](std::string s) { return std::stoi(s); });

  return std::vector(result.begin(), result.end());
}
