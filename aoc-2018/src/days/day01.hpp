#include <vector>
#include <string>
#include "../util/solution.hpp"


class Day01 : public Solution<int, std::vector<int>> {
public:
  int part1(const std::vector<int> &input) override;
  int part2(const std::vector<int> &input) override;

  std::vector<int> parse(const std::vector<std::string>& input) override;

  Day01() {
    this->DAY = "day01";
  }
};
