#include "string_extra.hpp"

std::vector<std::string> split(const std::string &s, char delim) {
    std::vector<std::string> elems;
    split_(s, delim, std::back_inserter(elems));
    return elems;
}
