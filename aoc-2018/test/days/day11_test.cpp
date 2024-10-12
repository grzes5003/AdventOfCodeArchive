#include "src/days/day11.hpp"
#include <gtest/gtest.h>

TEST(SupportMethods, BasicTC) {
  ASSERT_EQ(Day11::power({122,79}, Day11::Num(57)), -5);
  ASSERT_EQ(Day11::power({217,196}, Day11::Num(39)), 0);
  ASSERT_EQ(Day11::power({101,153}, Day11::Num(71)), 4);
}
