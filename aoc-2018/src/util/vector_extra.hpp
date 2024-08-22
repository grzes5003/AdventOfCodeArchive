#include <iterator>
#include <ostream>
#include <utility>
#include <vector>


template <class Iterable, class Begin = decltype(std::begin(std::declval<Iterable>()))>
inline std::ostream &operator<<(std::ostream &os, const Iterable &list) {
  os << "{";
  char const* div = "";
  for (const auto item : list) {
    os << div << item;
    div = ", ";
  }
  os << "}";
  return os;
}


