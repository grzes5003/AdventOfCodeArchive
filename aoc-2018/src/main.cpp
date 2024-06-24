#include "util/file_reader.hpp"
#include "days/day01.hpp"

int main(int argc, char *argv[]) {
  if (argc >= 2) {
    return 0;
  }

  Day01 day;
  day.solve();
  return 0;
}
