#include "day11.hpp"

using ::day11::Num;
// using ::Day11::Cache;
using ::day11::Coord;

namespace day11 {
std::ostream &operator<<(std::ostream &out, const Coord &coord) {
  out << "{" << coord.x << ", " << coord.y << "}" << std::endl;
  return out;
}
} // namespace day11

Num day11::power(const Coord &c, Num serial) {
  const Num rack_ID = c.x + 10;
  auto sum = (rack_ID * c.y + serial) * rack_ID;
  return static_cast<Num>(sum / 100) % 10 - 5;
}

static Num calculate_rect(const Coord &root, const Num &size,
                          const Num &serial) {
  Num result = 0;
  for (auto y = 0; y < size; ++y) {
    for (auto x = 0; x < size; ++x) {
      result += day11::power({root.x + x, root.y + y}, serial);
    }
  }
  return result;
}

Coord day11::Day11::part1(const Num &input) {
  const Num limit = 300;
  Num max_val = 0;
  Coord best_coord = {0, 0};
  for (auto y = 0; y < limit; ++y) {
    for (auto x = 0; x < limit; ++x) {
      auto result = calculate_rect({x, y}, 3, input);
      if (max_val < result) {
        max_val = result;
        best_coord = {x, y};
      }
    }
  }
  return best_coord;
}

Coord day11::Day11::part2(const Num &input) {
  const Num limit = 300;
  Num max_val = 0;
  Coord best_coord = {0, 0};
  for (auto size = 1; size <= limit; ++size) {
    for (auto y = 0; y < limit; ++y) {
      for (auto x = 0; x < limit; ++x) {
        auto result = calculate_rect({x, y}, size, input);
        if (max_val < result) {
          max_val = result;
          best_coord = {x, y};
          std::cout << x << "," << y << "," << size << std::endl;
        }
      }
    }
  }
  return best_coord;
}

Num day11::Day11::parse(const std::vector<std::string> &input) { return 1309; }
