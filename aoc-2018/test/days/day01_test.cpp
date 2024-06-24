#include <gtest/gtest.h>
#include <vector>
#include "../../src/days/day01.hpp"

// Demonstrate some basic assertions.
TEST(HelloTest, BasicAssertions) {
  // Expect two strings not to be equal.
  EXPECT_STRNE("hello", "world");
  // Expect equality.
  EXPECT_EQ(7 * 6, 42);
}

TEST(D01Task2, ExampleOne) {
  Day01 day01;
  std::vector<int> input = {
    1, -2, 3, 1, 1, -2
  };
  EXPECT_EQ(day01.part2(input), 2);
}


TEST(D01Task2, ExampleTwo) {
  Day01 day01;
  std::vector<int> input = {
    +3, +3, +4, -2, -4
  };
  EXPECT_EQ(day01.part2(input), 10);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
