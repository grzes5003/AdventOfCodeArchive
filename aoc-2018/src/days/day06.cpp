#include "day06.hpp"
#include <algorithm>
#include <array>
#include <climits>
#include <cstdlib>
#include <optional>
#include <ostream>
#include <queue>
#include <ranges>
#include <regex>
#include <set>
#include <string>

static constexpr std::array<Coord, 8> directions(Coord coord) {
  int x = coord.x;
  int y = coord.y;
  return {
      Coord(x + 1, y),     Coord(x - 1, y),     Coord(x, y + 1),
      Coord(x, y - 1),     Coord(x - 1, y - 1), Coord(x + 1, y - 1),
      Coord(x - 1, y + 1), Coord(x + 1, y + 1),
  };
}

int taxi_distance(const Coord &a, const Coord &b) {
  return std::abs(a.x - b.x) + std::abs(a.y - b.y);
}

std::ostream &operator<<(std::ostream &os, const Coord &coord) {
  os << "(" << coord.x << ", " << coord.y << ")";
  return os;
}

bool operator==(const Coord &a, const Coord &b) {
  return a.x == b.x && a.y == b.y;
}

bool operator<(const Coord &a, const Coord &b) {
  return a.x < b.x || (a.x == b.x && a.y < b.y);
}

int Day06::part1(const std::vector<Coord> &input) {
  int max = 0;
  for (const auto item : input) {
    int size = size_of(input, item).value_or(0);
    std::cout << item << " " << size << std::endl;
    max = max < size ? size : max;
  }
  return max;
}

int Day06::part2(const std::vector<Coord> &input) {
  const int limit = 10'000;
  Borders limits = {
      {input_borders.leftup.x - limit, input_borders.leftup.y - limit},
      {input_borders.rightdown.x + limit, input_borders.rightdown.y + limit}};

  int result = 0;
  for (auto x = limits.leftup.x; x < limits.rightdown.x; ++x) {
    for (auto y = limits.leftup.y; y < limits.rightdown.y; ++y) {
      int sum = 0;
      for (auto it = input.begin(), end = input.end(); it != end && sum < limit; ++it) {
        sum += taxi_distance(*it, {x, y});
      }
      if (sum < limit) {
        result += 1;
      }
    }
  }

  return result;
}

std::vector<Coord> Day06::parse(const std::vector<std::string> &input) {
  std::regex re("^(\\d+),\\s(\\d+)$");
  auto result = input | std::views::transform([=](std::string s) {
                  std::smatch match;
                  std::regex_search(s, match, re);

                  return Coord(std::stoi(match[1]), std::stoi(match[2]));
                });

  auto v_result = std::vector(result.begin(), result.end());
  this->input_borders = borders(v_result);
  return v_result;
}

std::optional<int> Day06::size_of(const std::vector<Coord> &coords,
                                  const Coord &coord) {
  std::set<Coord> seen;
  std::queue<Coord> queue;
  int size = -1;
  queue.push(coord);

  while (!queue.empty()) {
    auto current = queue.front();
    queue.pop();
    size += 1;

    auto neighbours = valid_neigh(coords, current, coord);
    if (!neighbours.has_value()) {
      return std::nullopt;
    }
    for (const auto neighbour : neighbours.value()) {
      if (!seen.contains(neighbour)) {
        seen.insert(neighbour);
        queue.push(neighbour);
      }
    }
  }
  return size;
}

std::optional<std::vector<Coord>>
Day06::valid_neigh(const std::vector<Coord> &other_coords, const Coord &current,
                   const Coord &target_coord) {
  auto candidates = directions(current);
  std::vector<Coord> result;
  for (const auto candidate : candidates) {
    auto is_inside_borders = candidate.x <= this->input_borders.rightdown.x &&
                             candidate.x >= this->input_borders.leftup.x &&
                             candidate.y <= this->input_borders.rightdown.y &&
                             candidate.y >= this->input_borders.leftup.y;
    if (!is_inside_borders) {
      return std::nullopt;
    }
    auto is_closest =
        std::none_of(other_coords.begin(), other_coords.end(), [=](auto coord) {
          if (coord == target_coord) {
            return false;
          }
          return taxi_distance(candidate, coord) <=
                 taxi_distance(candidate, target_coord);
        });
    if (is_closest) {
      result.push_back(candidate);
    }
  }
  return result;
}

Borders Day06::borders(const std::vector<Coord> &coords) {
  int x_min = INT_MAX, x_max = INT_MIN;
  int y_min = INT_MAX, y_max = INT_MIN;
  for (const auto coord : coords) {
    x_min = x_min > coord.x ? coord.x : x_min;
    x_max = x_max < coord.x ? coord.x : x_max;
    y_min = y_min > coord.y ? coord.y : y_min;
    y_max = y_max < coord.y ? coord.y : y_max;
  }

  return {{x_min, y_min}, {x_max, y_max}};
}
