#include "src/util/solution.hpp"
#include <cstdint>
#include <string>
#include <map>
#include <vector>


using Tree = std::map<char, std::vector<char>>;


struct Job {
  char c;
  uint8_t time_left;

public:
  static Job empty() {
    return Job('.', UINT8_MAX);
  }
};

struct Trees {
  Tree base;
  Tree reverse;
};

class Day07 : public Solution<std::string, Trees> {
  public:
  std::string part1(const Trees &input) override;
  std::string part2(const Trees &input) override;

  Trees parse(const std::vector<std::string> &input) override;

  Day07() { this->DAY = "day07"; }
};
