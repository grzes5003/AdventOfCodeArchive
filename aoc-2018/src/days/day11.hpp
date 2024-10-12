#include "src/util/solution.hpp"
#include <cstdint>


namespace day11 {

using Num = int32_t;

const Num SERIAL = 101;

struct Coord {
  Num x;
  Num y;
};

std::ostream &operator<<(std::ostream &out, const day11::Coord &coord);

Num power(const Coord &c, Num serial);

class Day11 : public Solution<Coord, Num> {
  Num grid_serial;
  Num limit = 300;

  Coord part1(const Num &input) override;
  Coord part2(const Num &input) override;

  Num parse(const std::vector<std::string> &input) override;

public:
  Day11() {
    this->DAY = "day11";
    this->grid_serial = 18;
  }
};

}; // namespace Day11
