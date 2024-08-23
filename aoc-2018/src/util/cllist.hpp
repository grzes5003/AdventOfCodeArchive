#ifndef CLLIST_HPP
#define CLLIST_HPP

#include <cstdint>
#include <memory>
#include <optional>

template <typename T> struct Node {
  public:
  T value;
  std::shared_ptr<Node> prev, next;
};

template <typename T>
class CircularLinkedList {
public:
  // friend std::ostream &operator<<(std::ostream &out, const CircularLinkedList<T> &list);

  uint64_t len;
  std::shared_ptr<Node<T>> current;
// public:
  void insert(T value);
  std::optional<T> get();
  T pop();

  void advance(int i);

  CircularLinkedList<T>() { len = 0; };
};

#include "cllist.cpp"

#endif // !CLLIST_HPP
