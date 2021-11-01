#include <deque>
#include <string>
#include <string_view>
#include <vector>

class aho_corasick {
  struct node {
    // children 组织出了 trie 树的结构
    // failure 是 Aho-Corasick 自动机的失效指针
    node *children[26], *failure;
    // accepting 表示该节点是多少个字符串的终点
    // 考虑到模式串可能会有相同输入的情况
    // 使用 accpeting 记录重复次数
    size_t accepting;

    // pattern 用于储存当前节点的实际字符串
    // 如果用不到应当删掉，构造它会导致复杂度变成 O(n²)
    // 另外如果字符串本身不会被释放（比如在 aho_corasick 内部持有所有模式串）
    // 使用引用类型 std::string_view 效率的构造可以保持 O(n)
    std::string pattern;

    node()
        : children()
        , accepting() {
    }

    ~node() {
      for (size_t k = 0; k != 26; ++k) delete this->children[k];
    }
  } root;
  // failure_was_built 标记自动机的失效指针是否已经构建
  // 因为每次 trie 中插入新串自动机就要重构
  bool failure_was_built;

  // build_failure 一次性构造自动机所有节点的失效指针
  void build_failure() {
    if (this->failure_was_built) return;
    std::deque<aho_corasick::node *> queue;
    this->root.failure = &this->root;
    // 初始化第一层节点，使得第一层节点失效指针均指向根节点
    // 注：若只初始化根节点，后续广搜会使得自动机上所有节点的失效指针都指向自己
    for (size_t k = 0; k != 26; ++k) {
      if (this->root.children[k] == nullptr) continue;
      this->root.children[k]->failure = &this->root;
      queue.push_back(this->root.children[k]);
    }
    // 执行广搜，对于每一个节点，对于其第 k 个孩子
    // 沿着其失效链寻找同时存在 k 孩子的失效节点
    while (!queue.empty()) {
      aho_corasick::node *n = queue.front();
      queue.pop_front();
      for (size_t k = 0; k != 26; k++) {
        if (n->children[k] == nullptr) continue;
        aho_corasick::node *p = n->failure;
        // p 首先指向 n 的失效节点，后续会沿着失效链走到根
        // 或者在失效链上找到也有一个 k 孩子的节点，
        // 这意味着找到了新的相匹配前后缀，逻辑类似 KMP
        while (p != &this->root && p->children[k] == nullptr) p = p->failure;
        if (p->children[k] != nullptr) p = p->children[k];
        n->children[k]->failure = p;
        queue.push_back(n->children[k]);
      }
    }
  }

public:
  aho_corasick() = default;
  ~aho_corasick() = default;

  // 构造 Trie 树
  void push(const std::string pattern) {
    this->failure_was_built = false;
    aho_corasick::node *p = &this->root;
    size_t index = 0;
    for (size_t index = 0; index != pattern.size(); ++index) {
      char c = pattern[index];
      // assert(is_lower(c));
      if (p->children[c - 'a'] == nullptr) {
        p->children[c - 'a'] = new aho_corasick::node();
        p->children[c - 'a']->pattern = pattern.substr(0, index);
      }
      p = p->children[c - 'a'];
    }
    ++p->accepting;
  }

  // search 方法从母串中搜索自动机中的所有模式串，返回的 results
  // 即为所有匹配串。
  std::vector<std::string_view> search(const std::string_view text) {
    this->build_failure();
    std::vector<std::string_view> result;
    aho_corasick::node *p = &this->root;
    for (char c: text) {
      // assert(islower(c));
      size_t k = c - 'a';
      // 若失配则沿着失配链走到根
      while (p != &this->root && p->children[k] == nullptr) p = p->failure;
      if (p->children[k] != nullptr) p = p->children[k];
      // 根的 accepting 必然是 0（排除空串的情况）
      // 对于每个走到的节点，都表示匹配到了以该节点为终点的模式串
      // 那么该模式串出现了多少次就 push 多少次即可
      // 也有可能只是计数匹配了多少次，也很容易改造
      for (; p->accepting; --p->accepting) result.push_back(p->pattern);
    }
    return result;
  }
};
