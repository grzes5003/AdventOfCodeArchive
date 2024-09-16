#include "src/days/day02.hpp"
#include <gtest/gtest.h>
#include <vector>

std::vector<std::vector<char>>
transform(const std::vector<std::string> &strings) {
  std::vector<std::vector<char>> result;

  for (const auto &str : strings) {
    std::vector<char> chars(str.begin(), str.end());
    result.push_back(chars);
  }

  return result;
}

TEST(D02Task, ExampleOne) {
  Day02 day02;
  std::vector<std::string> input = {"abcdef", "bababc", "abbcde", "abcccd",
                                    "aabcdd", "abcdee", "ababab"};
  std::vector<std::vector<char>> result = transform(input);
  EXPECT_EQ(day02.part1(result), 12);
}
