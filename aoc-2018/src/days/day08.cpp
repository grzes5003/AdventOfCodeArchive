#include "day08.hpp"
#include "src/util/string_extra.hpp"
#include "src/util/vector_extra.hpp"
#include <algorithm>
#include <cstdint>
#include <functional>
#include <iterator>
#include <numeric>
#include <ostream>
#include <string>
#include <utility>
#include <vector>

std::ostream &operator<<(std::ostream &out, const Node &node) {
  out << "{";
  for (auto it = node.begin; it != node.end; std::advance(it, 1)) {
    out << (int)*it << ", ";
  }
  out << "}" << std::endl;
  return out;
}

std::vector<uint8_t> Day08::parse(const std::vector<std::string> &input) {
  std::vector<std::string> vec = split(input[0], ' ');
  std::vector<uint8_t> result(vec.size());
  std::transform(vec.begin(), vec.end(), result.begin(), [](const auto &el) {
    return static_cast<uint8_t>(std::stoi(el));
  });
  return result;
}

Node Node::parse(Vec::const_iterator &iter,
                 std::map<Vec::const_iterator, Node> &cache,
                 std::map<Vec::const_iterator, std::vector<Vec::const_iterator>>
                     &parents_cache) {
  auto it = cache.find(iter);
  if (it != cache.end()) {
    iter = it->first;
    return it->second;
  }

  Vec::const_iterator begin = iter;
  uint8_t n_children = *iter;
  std::advance(iter, 1);
  uint8_t n_metadata = *iter;
  std::advance(iter, 1);

  parents_cache.insert(
      std::make_pair(begin, std::vector<Vec::const_iterator>()));

  auto remaining = n_children;
  while ((remaining--) > 0) {
    auto _begin = iter;
    Node child = Node::parse(iter, cache, parents_cache);
    cache.insert(std::make_pair(_begin, child));

    auto keyval = parents_cache.find(begin);
    keyval->second.push_back(child.begin);
  }

  Vec::const_iterator metadata = iter;
  std::advance(iter, n_metadata);
  Vec::const_iterator end = iter;
  Node result = Node(n_metadata, n_metadata, begin, metadata, end);
  cache.insert(std::make_pair(begin, result));
  return result;
}

void Day08::populate(const std::vector<uint8_t> &input) {
  auto iter = input.begin();
  Node::parse(iter, cache, parents_cache);
}

int Day08::part1(const std::vector<uint8_t> &input) {
  populate(input);
  int sum = 0;
  for (const auto &item : cache) {
    sum += std::accumulate(item.second.metadata, item.second.end, 0,
                           std::plus<int>());
  }
  return sum;
}

int Day08::calc_value(const Node &node) {
  auto children = parents_cache.find(node.begin)->second;
  if (children.empty()) {
    return std::accumulate(node.metadata, node.end, 0, std::plus<int>());
  }
  int sum = 0;
  for (auto iter = node.metadata; iter != node.end; std::advance(iter, 1)) {
    if (*iter <= children.size()) {
      auto child_node = cache.find(children[*iter - 1])->second;
      sum += calc_value(child_node);
    }
  }
  return sum;
}

int Day08::part2(const std::vector<uint8_t> &input) {
  auto root = cache.find(input.begin())->second;
  return calc_value(root);
}
