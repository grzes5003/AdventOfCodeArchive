#include "../util/solution.hpp"
#include <string>
#include <vector>
#include <map>

using Input = std::vector<std::vector<char>>;

class Day02 : public Solution<int, Input> {
public:
  int part1(const Input &input) override;
  int part2(const Input &input) override;

  Input parse(std::vector<std::string> input) override;

  Day02() { this->DAY = "day02"; }

private:
  static std::map<char, int> frequencies(const std::vector<char> &vec);
  static int count_freq(const std::map<char, int> &freq, int target);
};
