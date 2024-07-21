#include "day05.hpp"
#include <algorithm>
#include <cctype>
#include <climits>
#include <iterator>
#include <locale>
#include <ostream>
#include <set>
#include <vector>

template <typename T>
std::ostream &operator<<(std::ostream &os, const std::list<T> &list) {
  os << "\"";
  for (const auto item : list) {
    os << item;
  }
  os << "\"";
  return os;
}

static std::set<char> list_units(const std::list<char> &list) {
  std::set<char> stage(list.begin(), list.end());
  std::set<char> result;
  for (const auto item : stage) {
    auto lower = std::tolower(item, std::locale());
    auto upper = std::toupper(item, std::locale());
    if (stage.contains(lower) && stage.contains(upper)) {
      result.insert(lower);
    }
  }
  return result;
}

static std::list<char> filter_unit(const std::list<char> &list,
                                   const char &val) {
  std::list<char> result;
  std::copy_if(
      list.begin(), list.end(), std::back_inserter(result),
      [&](auto item) { return val != std::tolower(item, std::locale()); });
  return result;
}

static bool protein_cond(char a, char b) {
  if (isupper(a) && islower(b)) {
    return std::tolower(a, std::locale()) == b;
  } else if (isupper(b) && islower(a)) {
    return std::tolower(b, std::locale()) == a;
  }
  return false;
}

static void reduce(std::list<char> &list) {
  auto left = list.begin();
  auto right = list.begin();
  std::advance(right, 1);

  while (right != list.end()) {
    if (protein_cond(*left, *right)) {
      right = list.erase(right);
      left = list.erase(left);
    } else {
      std::advance(left, 1);
    }
    if (right != list.end()) {
      std::advance(right, 1);
    }
  }
}

int Day05::part1(const std::list<char> &input) {
  auto len = input.size();
  auto protein = input;
  while (true) {
    reduce(protein);
    if (protein.size() == len) {
      break;
    }
    len = protein.size();
  }
  return len;
}

int Day05::part2(const std::list<char> &input) {
  auto units = list_units(input);
  int max = INT_MAX;
  for (const auto unit : units) {
    auto candidate_list = filter_unit(input, unit);
    auto len = candidate_list.size();
    while (true) {
      reduce(candidate_list);
      if (candidate_list.size() == len) {
        break;
      }
      len = candidate_list.size();
    }
    if (max > len) {
      max = len;
    }
  }
  return max;
}

std::list<char> Day05::parse(const std::vector<std::string> &input) {
  if (input.size() != 1) {
    return {};
  }
  return std::list<char>(input[0].begin(), input[0].end());
}
