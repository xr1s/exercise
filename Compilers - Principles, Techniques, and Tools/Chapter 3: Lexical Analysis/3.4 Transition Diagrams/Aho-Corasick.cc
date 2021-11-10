#include <string_view>
#include <vector>

// 为了方便起见，没有做更多的抽象；一些可以做的抽象包括
// 一、
//     使用模板参数确定 children 数组的长度，不要写死 26；
//     此时 push 和 search 方法就应该传入一对解引用为整数的 InputIterator；
//     就不需要由 aho_corasick 将每个字符映射为对应的 child，而是交给用户。
//     这样可以将 aho_corasick 自动机拓展到 26 个小写字母之外。
//     这一步将 aho_corasick 与 char 字符串相解耦。
// 二、
//     aho_corasick::node 不应该直接以指针暴露出来，应当实现一个包装类。
class aho_corasick {
public:
  class node {
  private:
    // children 组织出了 Trie 树的结构；
    // failure 是 Aho-Corasick 自动机的失效指针。
    node *children[26], *failure;
    // match_count 是该节点在一次 search 中被实际匹配到的次数；
    // 它需要在每次 search 前清空，否则会累加上次 search 的结果。
    size_t match_count;
    friend aho_corasick;
    node()
        : children() {
    }

  public:
    // match_times 暴露出只读接口，用于输出某字符串被匹配到多少次；
    size_t match_times() {
      return this->match_count;
    }
  };

private:
  aho_corasick::node root;
  // failure_was_built 标记自动机的失效指针是否已经构建；
  // 因为每次 trie 中插入新串自动机就要重构失效链。
  bool failure_was_built;
  // queue 是对 Trie 进行广度优先搜索的结果；
  // 下推 match_count 和析构整棵 Trie 用到了该结果。
  // 关于下推 match_count 见 search 函数的最后两行。
  std::vector<aho_corasick::node *> queue;

  // 通过对 Trie 进行广度优先搜索，
  // 一次性构造所有节点的失效指针；
  // 同时储存该树的广搜序于 queue 中。
  void build_failure() {
    if (this->failure_was_built) return;
    const auto root = &this->root;
    root->failure = root;
    std::vector<aho_corasick::node *> queue;
    // 初始化第一层节点，使得第一层节点失效指针均指向根节点；
    // 注：若只初始化根节点直接广搜，会使得自动机上所有节点的失效指针都指向自己
    for (int k = 0; k != 26; ++k)
      if (this->root.children[k] != nullptr) {
        this->root.children[k]->failure = root;
        queue.push_back(this->root.children[k]);
      }
    // 执行广搜，对于每一个节点，对于其第 k 个孩子，
    // 沿着该节点的失效链寻找存在 k 孩子的失效节点。
    for (size_t head = 0; head != queue.size(); ++head) {
      auto n = queue[head];
      for (size_t k = 0; k != 26; ++k) {
        if (n->children[k] == nullptr) continue;
        // p 首先指向 n 的失效节点，后续会沿着 n 的失效链走到根，
        // 或者在失效链上碰到也有一个 k 孩子的节点，
        // 这意味着找到了新的相匹配前后缀，逻辑类似 KMP。
        auto p = n->failure;
        while (p != root && p->children[k] == nullptr) p = p->failure;
        if (p->children[k] != nullptr) p = p->children[k];
        n->children[k]->failure = p;
        queue.push_back(n->children[k]);
      }
    }
    // failure_was_built 是 false 表明有新节点被插入，
    // 说明 Trie 结构已经被改变了，广搜序也需要被替换。
    this->queue = std::move(queue);
  }

  // 在下一次搜索前清理上次搜索的 match_count，
  // 防止累加上次的结果。
  void clear_match_count() {
    for (auto n: this->queue) n->match_count = 0;
  }

public:
  ~aho_corasick() {
    this->build_failure();
    // 整理出所有节点后进行析构；
    // 递归析构容易导致栈溢出。
    for (auto n: this->queue) delete n;
  }

  // push 单个字符串以构造 Trie 树。
  // 返回值是接受节点的指针，用于用户储存并最终计算模式串的匹配次数。
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

  // search 方法从母串中搜索自动机中的所有模式串，
  void search(const std::string_view text) {
    this->build_failure();
    this->clear_match_count();
    const auto root = &this->root;
    auto n = root;
    for (auto c: text) {
      size_t k = c - 'a';
      while (n != root && n->children[k] == nullptr) n = n->failure;
      if (n->children[k] != nullptr) n = n->children[k];
      // 当前节点匹配到了母串，则将匹配次数增加一次；
      // 当某节点匹配到了的时候，其失效链上所有节点均会匹配，均应更新；
      // 但这里采用惰性更新的方法，在整个母串匹配完后一次性下推标记即可；
      n->match_count = n->match_count + 1;
    }
    // 根据广搜序，从叶子节点往根走，一次性下推（沿树上推，沿失效链下推）所有标记；
    for (size_t k = this->queue.size() - 1; ~k; --k)
      this->queue[k]->failure->match_count += this->queue[k]->match_count;
  }
};
