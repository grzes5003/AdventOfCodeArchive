#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include "file_reader.hpp"
// using Inp = std::vector<std::string>

template <typename T, typename I> class Solution {
public:
  virtual T part1(const I &input) = 0;
  virtual T part2(const I &input) = 0;

  virtual I parse(const std::vector<std::string>& raw_input) = 0;

  void solve() {
    std::stringstream ss;
    ss << "input\\" << DAY << ".in";

    auto result = read_lines(ss.str());
    if (!result.has_value()) {
      throw std::invalid_argument("cannot read the file");
    }
    auto parsed = parse(result.value());

    auto solution1 = part1(parsed);
    auto solution2 = part2(parsed);
    std::cout << "(" << solution1 << "," << solution2 << ")\n";
  }

  std::string DAY;
};
