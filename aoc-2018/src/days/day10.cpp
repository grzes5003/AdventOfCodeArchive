
#include "day10.hpp"
#include <algorithm>
#include <iterator>
#include <ostream>
#include <regex>
#include <set>
#include <vector>

template <typename T> struct Limits {
  T max_x;
  T min_x;
  T max_y;
  T min_y;
};

template <typename T>
static Limits<T> limits(const std::vector<Record<T>> &records) {
  auto max_x = (*std::max_element(records.begin(), records.end(),
                                  [](const auto &a, const auto &b) {
                                    return a.pos.x < b.pos.x;
                                  }))
                   .pos.x;

  auto min_x = (*std::min_element(records.begin(), records.end(),
                                  [](const auto &a, const auto &b) {
                                    return a.pos.x < b.pos.x;
                                  }))
                   .pos.x;

  auto max_y = (*std::max_element(records.begin(), records.end(),
                                  [](const auto &a, const auto &b) {
                                    return a.pos.y < b.pos.y;
                                  }))
                   .pos.y;

  auto min_y = (*std::min_element(records.begin(), records.end(),
                                  [](const auto &a, const auto &b) {
                                    return a.pos.y < b.pos.y;
                                  }))
                   .pos.y;
  return Limits(max_x, min_x, max_y, min_y);
}

template <typename T>
std::ostream &operator<<(std::ostream &os,
                         const std::vector<Record<T>> &records) {

  Limits<T> _limits = limits(records);

  std::set<Vec2<T>> positions;
  std::transform(records.begin(), records.end(),
                 std::inserter(positions, positions.begin()),
                 [](const auto &record) { return record.pos; });

  for (auto y = _limits.min_y - 2; y < _limits.max_y + 1; ++y) {
    for (auto x = _limits.min_x - 2; x < _limits.max_x + 2; ++x) {
      os << (positions.contains(Vec2<T>(x, y)) ? "x" : ".");
    }
    os << std::endl;
  }
  return os;
}

std::vector<Record<Int>> Day10::parse(const std::vector<std::string> &input) {
  std::vector<Record<Int>> records;
  std::regex re(".*<\\s*(-?\\d+),\\s*(-?\\d+)>.*<\\s*(-?\\d+),\\s*(-?\\d+)>");
  for (const auto &line : input) {
    std::smatch match;
    std::regex_search(line, match, re);
    records.push_back(
        Record<Int>::from_strings(match[1], match[2], match[3], match[4]));
  }
  return records;
}

template <typename T> Record<T> Record<T>::deter() const {
  return Record<T>(Vec2<T>(pos.x - velocity.x, pos.y - velocity.y), velocity);
}

template <typename T> Record<T> Record<T>::iter() const {
  return Record<T>(Vec2<T>(pos.x + velocity.x, pos.y + velocity.y), velocity);
}

template <typename T>
Record<T> Record<T>::from_strings(std::string a, std::string b, std::string c,
                                  std::string d) {
  return Record<T>(Vec2<T>(std::stoi(a), std::stoi(b)),
                   Vec2<T>(std::stoi(c), std::stoi(d)));
}

static int simulate(std::vector<Record<Int>> &records) {
  Limits<Int> old_limits = limits(records);
  int timer = 0;
  while (++timer) {
    std::transform(records.begin(), records.end(), records.begin(),
                   [](const auto &record) { return record.iter(); });
    auto new_limits = limits(records);
    if (new_limits.max_x > old_limits.max_x ||
        new_limits.min_x < old_limits.min_x) {
      std::transform(records.begin(), records.end(), records.begin(),
                     [](const auto &record) { return record.deter(); });
      std::cout << records << std::endl;
      break;
    }
    old_limits = new_limits;
  }
  return timer - 1;
}

std::string Day10::part1(const std::vector<Record<Int>> &input) {
  std::vector<Record<Int>> records = input;
  return "";
}

std::string Day10::part2(const std::vector<Record<Int>> &input) {
  std::vector<Record<Int>> records = input;
  return std::to_string(simulate(records));
}
