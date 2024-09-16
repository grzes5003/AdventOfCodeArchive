#include "day09.hpp"
#include "src/util/cllist.hpp"
#include <algorithm>

Game Day09::parse(const std::vector<std::string> &input) {
  return Game(428, 70825);
}

int Game::move(int at, int offset) const {
  return at + offset % (board.size());
}

static unsigned long calculate(Game game) {
  CircularLinkedList<int> l;
  l.insert(0);
  std::vector<unsigned long> players_score(game.players);

  for (unsigned long marble_id = 1; marble_id <= game.last_marble;
       ++marble_id) {
    if (marble_id % 23 == 0) {
      l.advance(-7);
      players_score[marble_id % game.players] += marble_id + l.pop();
    } else {
      l.advance(1);
      l.insert(marble_id);
    }
  }
  return *std::max_element(players_score.begin(), players_score.end());
}

unsigned long Day09::part1(const Game &input) {
  Game game = input;
  return calculate(game);
}

unsigned long Day09::part2(const Game &input) {
  Game game = input;
  game.last_marble *= 100;
  return calculate(game);
}
