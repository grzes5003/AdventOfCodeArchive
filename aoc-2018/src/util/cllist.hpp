#ifndef CLLIST_HPP
#define CLLIST_HPP

#include <cstdint>
#include <memory>
#include <optional>

#include <iostream>
#include <ostream>

template <typename T> struct Node {
public:
  T value;
  std::shared_ptr<Node> prev, next;
};

template <typename T> class CircularLinkedList {
public:
  uint64_t len;
  std::shared_ptr<Node<T>> current;
  void insert(T value);
  std::optional<T> get();
  T pop();

  void advance(int i);

  CircularLinkedList<T>() { len = 0; };
};

template <typename T> int sgn(T val) { return (T(0) < val) - (val < T(0)); }

template <typename T>
std::ostream &operator<<(std::ostream &out, const CircularLinkedList<T> &list) {
  out << "{";
  if (list.current != nullptr) {
    auto current = list.current;
    do {
      out << current->value << ", ";
      current = current->next;
    } while (current->value != list.current->value);
  }
  out << "}";
  return out;
}

template <typename T> void CircularLinkedList<T>::insert(T value) {
  auto new_node = std::make_shared<Node<T>>(Node(value));
  len += 1;
  if (current == nullptr) {
    new_node->next = new_node;
    new_node->prev = new_node;
    current = new_node;
    return;
  }
  if (len == 2) {
    new_node->prev = current;
    new_node->next = current;
    current->next = new_node;
    current->prev = new_node;
    current = new_node;
    return;
  }
  new_node->prev = current;
  new_node->next = current->next;
  current->next->prev = new_node;
  current->next = new_node;
  current = new_node;
}

template <typename T> T CircularLinkedList<T>::pop() {
  if (current == nullptr) {
    return T();
  }
  auto value = current->value;
  len -= 1;
  current->prev->next = current->next;
  current->next->prev = current->prev;
  current = current->next;
  return value;
}

template <typename T> std::optional<T> CircularLinkedList<T>::get() {
  if (current == nullptr) {
    return {};
  }
  return current->value;
}

template <typename T> void CircularLinkedList<T>::advance(int i) {
  if (current == nullptr) {
    return;
  }
  auto counter = std::abs(i);
  while (counter--) {
    if (i <= 0) {
      current = current->prev;
    } else {
      current = current->next;
    }
  }
}

#endif // !CLLIST_HPP
