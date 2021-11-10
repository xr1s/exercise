#include <iostream>
#include <vector>

class aho_corasick {
public:
  class node {
  private:
    node *children[26], *failure;
    size_t match_count;
    friend aho_corasick;
    node()
        : children() {
    }

  public:
    size_t match_times() {
      return this->match_count;
    }
  };

private:
  aho_corasick::node root;
  bool failure_was_built;
  std::vector<aho_corasick::node *> queue;
  void build_failure() {
    if (this->failure_was_built) return;
    const auto root = &this->root;
    root->failure = root;
    std::vector<aho_corasick::node *> queue;
    for (int k = 0; k != 26; ++k)
      if (this->root.children[k] != nullptr) {
        this->root.children[k]->failure = root;
        queue.push_back(this->root.children[k]);
      }
    for (size_t head = 0; head != queue.size(); ++head) {
      auto n = queue[head];
      for (size_t k = 0; k != 26; ++k) {
        if (n->children[k] == nullptr) continue;
        auto p = n->failure;
        while (p != root && p->children[k] == nullptr) p = p->failure;
        if (p->children[k] != nullptr) p = p->children[k];
        n->children[k]->failure = p;
        queue.push_back(n->children[k]);
      }
    }
    this->queue = std::move(queue);
  }
  void clear_match_count() {
    for (auto n: this->queue) n->match_count = 0;
  }

public:
  ~aho_corasick() {
    this->build_failure();
    for (auto n: this->queue) delete n;
  }
  aho_corasick::node *push(const std::string_view pattern) {
    auto n = &this->root;
    for (auto c: pattern) {
      size_t k = c - 'a';
      if (n->children[k] == nullptr) {
        n->children[k] = new aho_corasick::node();
        this->failure_was_built = false;
      }
      n = n->children[k];
    }
    return n;
  }
  void search(const std::string_view text) {
    this->build_failure();
    this->clear_match_count();
    const auto root = &this->root;
    auto n = root;
    for (auto c: text) {
      size_t k = c - 'a';
      while (n != root && n->children[k] == nullptr) n = n->failure;
      if (n->children[k] != nullptr) n = n->children[k];
      n->match_count = n->match_count + 1;
    }
    for (size_t k = this->queue.size() - 1; ~k; --k)
      this->queue[k]->failure->match_count += this->queue[k]->match_count;
  }
};

int main() {
  std::ios_base::sync_with_stdio(false);
  std::cin.tie(nullptr), std::cout.tie(nullptr);
  size_t n;
  std::string s;
  s.reserve(1000000);

  while (std::cin >> n && n) {
    std::vector<std::string> patterns;
    std::vector<aho_corasick::node *> accepting;
    patterns.reserve(n);
    accepting.reserve(n);
    aho_corasick automaton;
    for (size_t k = 0; k != n; ++k) {
      std::cin >> s;
      auto node = automaton.push(s);
      patterns.emplace_back(std::move(s));
      accepting.emplace_back(node);
    }
    std::cin >> s;
    automaton.search(s);
    int max_count = 0;
    for (auto node: accepting)
      if (node->match_times() > max_count) {
        max_count = node->match_times();
      }
    std::cout << max_count << '\n';
    for (size_t k = 0; k != n; ++k)
      if (accepting[k]->match_times() == max_count)
        std::cout << patterns[k] << '\n';
  }
}
