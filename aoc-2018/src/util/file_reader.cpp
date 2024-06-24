#include "file_reader.hpp"
#include <fstream>
#include <iostream>
#include <ostream>


std::optional<std::vector<std::string>> read_lines(std::string filename) {
  std::ifstream file(filename);
  if(!file) {
    std::cerr << "Unable to open the file " << filename << std::endl;
    return std::nullopt;
  }

  std::vector<std::string> result;
  std::string line;
  while (std::getline(file, line)) {
    result.push_back(line);
  }
  file.close();
  return result;
}
