#include "../util/solution.hpp"
#include <cstdint>
#include <map>
#include <string>
#include <vector>

using Vec = std::vector<uint8_t>;

struct Node {
  uint32_t n_children;
  uint32_t n_metadata;
  Vec::const_iterator begin;
  Vec::const_iterator metadata;
  Vec::const_iterator end;

public:
  static Node
  parse(Vec::const_iterator &begin, std::map<Vec::const_iterator, Node> &cache,
        std::map<Vec::const_iterator, std::vector<Vec::const_iterator>>
            &parents_cache);
};

class Day08 : public Solution<int, std::vector<uint8_t>> {
  std::map<Vec::const_iterator, Node> cache;
  std::map<Vec::const_iterator, std::vector<Vec::const_iterator>> parents_cache;

public:
  int part1(const std::vector<uint8_t> &input) override;
  int part2(const std::vector<uint8_t> &input) override;

  std::vector<uint8_t> parse(const std::vector<std::string> &input) override;
  void populate(const std::vector<uint8_t> &input);
  int calc_value(const Node &node);

  Day08() { this->DAY = "day08"; }
};
