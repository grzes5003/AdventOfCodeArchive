#include "src/util/solution.hpp"
#include <vector>
#include <list>


class Day05 : public Solution<int, std::list<char>> {
public:
  int part1(const std::list<char> &input) override;
  int part2(const std::list<char> &input) override;

  std::list<char> parse(const std::vector<std::string>& input) override;

  Day05() { this->DAY = "day05"; }
};
