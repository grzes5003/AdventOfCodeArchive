
#include "src/util/solution.hpp"
#include <optional>
#include <vector>


struct Coord {
  int x;
  int y;
};

struct Borders {
  Coord leftup;
  Coord rightdown;
};


class Day06 : public Solution<int, std::vector<Coord>> {
public:
  int part1(const std::vector<Coord> &input) override;
  int part2(const std::vector<Coord> &input) override;

  std::vector<Coord> parse(const std::vector<std::string>& input) override;

  Day06() { this->DAY = "day06"; }
private:
  Borders input_borders;

  std::optional<int> size_of(const std::vector<Coord>& coords, const Coord& coord);
  std::optional<std::vector<Coord>> valid_neigh(const std::vector<Coord>& coords, const Coord& current, const Coord& target);
  Borders borders(const std::vector<Coord> &coords);
};
