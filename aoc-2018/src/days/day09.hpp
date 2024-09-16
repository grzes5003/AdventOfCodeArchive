#include "src/util/solution.hpp"
#include <vector>

using Board = std::vector<int>;

struct Game {
  unsigned int players;
  unsigned long last_marble;
  Board board;

public:
  int move(int at, int offset) const;
  Game(unsigned int players, unsigned long last_marble)
      : players(players), last_marble(last_marble), board({0}){};
};

class Day09 : public Solution<unsigned long, Game> {
public:
  unsigned long part1(const Game &input) override;
  unsigned long part2(const Game &input) override;

  Game parse(const std::vector<std::string> &input) override;

  Day09() { this->DAY = "day09"; }
};
