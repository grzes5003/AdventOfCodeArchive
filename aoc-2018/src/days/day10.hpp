#include "../util/solution.hpp"
#include <cstdint>
#include <string>
#include <vector>

using Int = int32_t;

template <typename T> struct Vec2 {
  T x;
  T y;

  bool operator<(const Vec2 other) const noexcept {
    return x < other.x || (x == other.x && y < other.y);
  }
};

template <typename T> struct Record {
public: 
  Vec2<T> pos;
  Vec2<T> velocity;
  static Record from_strings(std::string a, std::string b, std::string c, std::string d);
  Record deter() const;
  Record iter() const;
};

class Day10 : public Solution<std::string, std::vector<Record<Int>>> {

public:
  std::string part1(const std::vector<Record<Int>> &input) override;
  std::string part2(const std::vector<Record<Int>> &input) override;

  std::vector<Record<Int>>
  parse(const std::vector<std::string> &input) override;

  Day10() { this->DAY = "day10"; }
};
