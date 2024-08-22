#include "day07.hpp"
#include "../util/string_extra.hpp"
#include "../util/vector_extra.hpp"
#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>
#include <iterator>
#include <optional>
#include <ostream>
#include <set>
#include <utility>
#include <vector>

std::ostream &operator<<(std::ostream &out, const Job &job) {
  out << "{c=" << job.c << ", tl=" << (int)job.time_left << "}";
  return out;
}

std::ostream &operator<<(std::ostream &out, const Tree &tree) {
  for (auto item : tree) {
    out << item.first << ": ";
    for (auto l : item.second) {
      out << l << ",";
    }
    out << std::endl;
  }
  return out;
}

static std::optional<std::pair<char, char>>
extract_nums(const std::string line) {
  auto v_words = split(line, ' ');
  if (v_words.size() < 8) {
    return {};
  }
  return std::make_pair(static_cast<char>(v_words[1][0]),
                        static_cast<char>(v_words[7][0]));
}

static void append_value(Tree &tree, std::pair<char, char> pair, bool reverse) {
  char l = reverse ? pair.second : pair.first;
  char r = reverse ? pair.first : pair.second;

  Tree::iterator it = tree.find(l);
  if (it != tree.end()) {
    it->second.push_back(r);
  } else {
    tree.insert(std::make_pair(l, std::vector<char>{r}));
  }
}

Trees Day07::parse(const std::vector<std::string> &input) {
  Tree base;
  Tree reverse;
  for (const auto &line : input) {
    auto deps = extract_nums(line);
    if (deps.has_value()) {
      append_value(base, deps.value(), false);
      append_value(reverse, deps.value(), true);
    }
  }
  return {base, reverse};
}

std::set<char> find_roots(const Tree &tree) {
  std::set<char> seen;
  std::set<char> keys;
  for (const auto &keyval : tree) {
    keys.insert(keyval.first);
    for (const auto &item : keyval.second) {
      seen.insert(item);
    }
  }
  std::set<char> result;
  std::set_difference(keys.begin(), keys.end(), seen.begin(), seen.end(),
                      std::inserter(result, result.end()));
  return result;
}

template <std::size_t SIZE>
std::array<std::optional<Job>, SIZE>::iterator
find_free_worker(std::array<std::optional<Job>, SIZE> &workers) {
  return std::find_if(workers.begin(), workers.end(),
                      [](const auto &q) { return !q.has_value(); });
}

bool is_ready(const Trees &trees, const std::set<char> &seen, char c) {
  const auto v_req = trees.reverse.find(c)->second;
  const auto required = std::set<char>(v_req.begin(), v_req.end());
  std::set<char> result;
  std::set_difference(required.begin(), required.end(), seen.begin(),
                      seen.end(), std::inserter(result, result.begin()));

  return result.empty();
}

template <std::size_t SIZE>
bool is_pending(const Trees &trees,
                const std::array<std::optional<Job>, SIZE> workers, char c) {
  const auto v_req = trees.reverse.find(c)->second;
  const auto required = std::set<char>(v_req.begin(), v_req.end());

  return std::any_of(
      workers.begin(), workers.end(),
      [c, &required = std::as_const(required)](const auto &worker) {
        return required.contains(worker.value_or(Job::empty()).c);
      });
}

void sort_queue(std::vector<char> &queue) {
  std::sort(queue.begin(), queue.end());
}

std::string Day07::part1(const Trees &trees) {
  std::string result;
  std::set<char> roots_set = find_roots(trees.base);
  std::vector<char> queue =
      std::vector<char>(roots_set.begin(), roots_set.end());
  sort_queue(queue);
  std::set<char> seen = {queue.front()};

  while (queue.size()) {
    auto current = queue.front();
    queue.erase(queue.begin());
    seen.insert(current);
    result.push_back(current);

    auto current_keyval = trees.base.find(current);
    if (current_keyval == trees.base.end()) {
      continue;
    }
    auto neighbours = current_keyval->second;
    for (const auto &neigh : neighbours) {
      if (!seen.contains(neigh) && is_ready(trees, seen, neigh)) {
        queue.push_back(neigh);
      }
    }
    sort_queue(queue);
  }

  return result;
}

std::string Day07::part2(const Trees &trees) {
  std::cout << trees.reverse << std::endl;

  const uint8_t workers = 5;
  const uint8_t additional_time = 60;

  std::array<std::optional<Job>, workers> workers_queues;

  std::string result;
  std::set<char> roots_set = find_roots(trees.base);
  std::vector<char> queue =
      std::vector<char>(roots_set.begin(), roots_set.end());
  sort_queue(queue);
  std::set<char> seen = {queue.front()};

  uint16_t total_time = 0;
  while (true) {
    for (auto &worker : workers_queues) {
      if (worker && worker->time_left == 0) {
        result.push_back(worker.value().c);
        auto current_keyval = trees.base.find(worker.value().c);
        worker = std::nullopt;
        if (current_keyval == trees.base.end()) {
          continue;
        }

        auto neighbours = current_keyval->second;
        for (const auto &neigh : neighbours) {
          if (!seen.contains(neigh) && is_ready(trees, seen, neigh) &&
              !is_pending(trees, workers_queues, neigh)) {
            queue.push_back(neigh);
          }
        }
      }
    }
    sort_queue(queue);

    auto empty_worker = find_free_worker(workers_queues);
    if (empty_worker != workers_queues.end() && !queue.empty()) {
      auto current = queue.front();
      queue.erase(queue.begin());
      seen.insert(current);
      *empty_worker =
          std::make_optional(Job(current, (int)current + additional_time - 64));
    } else if (std::all_of(workers_queues.begin(), workers_queues.end(),
                           [](const auto &q) { return !q.has_value(); }) &&
               queue.empty()) {
      break;
    } else {
      Job max_job = {'.', UINT8_MAX};
      auto min_worker =
          std::min_element(workers_queues.begin(), workers_queues.end(),
                           [&max_job](const auto &a, const auto &b) {
                             return a.value_or(max_job).time_left <
                                    b.value_or(max_job).time_left;
                           })
              ->value()
              .time_left;
      total_time += min_worker;
      for (auto &worker : workers_queues) {
        if (worker) {
          worker->time_left -= min_worker;
        }
      }
    }
  }
  std::cout << "res: " << total_time << std::endl;
  return result;
}
