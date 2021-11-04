#include <deque>
#include <map>
#include <string>
#include <string_view>
#include <vector>

class aho_corasick {
  struct node {
    // children 组织出了 Trie 树的结构；
    // failure 是 Aho-Corasick 自动机的失效指针。
    node *children[26], *failure;
    // pattern 用于在 Trie 接受节点标记该节点对应的模式串；
    // 当且仅当该节点在 Trie 上表示的串出现在模式串中，该值才非空。
    std::string pattern;
    // accepting 表示有多少字符串的接受状态在当前节点；
    // 换句话说就是当前节点在 Trie 树上表示的串在模式串集合中出现了几次。
    size_t accepting;
    // times 表示当前使用的母串搜索时匹配到了多少次该状态；
    // 该值会在每次搜索后清空，防止影响到下次搜索。
    size_t times;
    // indegree 表示有多少失效指针指向当前节点；
    // 用于拓扑排序将每个节点的匹配次数沿失效链下推。
    size_t indegree;

    node()
        : accepting()
        , children()
        , indegree()
        , times() {
    }

    ~node() {
      for (auto child: this->children) delete child;
    }
  } root;
  // failure_was_built 标记自动机的失效指针是否已经构建
  // 因为每次 trie 中插入新串自动机就要重构
  bool failure_was_built;

  // build_failure 一次性构造自动机所有节点的失效指针
  void build_failure() {
    if (this->failure_was_built) return;
    this->failure_was_built = true;
    std::deque<aho_corasick::node *> queue;
    this->root.failure = &this->root;
    // 初始化第一层节点，使得第一层节点失效指针均指向根节点
    // 注：若只初始化根节点，后续广搜会使得自动机上所有节点的失效指针都指向自己
    for (auto child: this->root.children) {
      if (child == nullptr) continue;
      child->failure = &this->root;
      queue.push_back(child);
    }
    // 执行广搜，对于每一个节点，对于其第 k 个孩子
    // 沿着其失效链寻找同时存在 k 孩子的失效节点
    while (!queue.empty()) {
      auto n = queue.front();
      queue.pop_front();
      for (size_t k = 0; k != 26; k++) {
        if (n->children[k] == nullptr) continue;
        auto p = n->failure;
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

  void topological_sort_failure(std::map<std::string_view, size_t> &results) {
    std::deque<aho_corasick::node *> queue;
    std::deque<aho_corasick::node *> topological_sort_queue;
    // 一、计算所有节点的失效节点入度
    queue.push_back(&this->root);
    ++this->root.indegree;
    while (!queue.empty()) {
      auto n = queue.front();
      queue.pop_front();
      for (auto child: n->children) {
        if (child == nullptr) continue;
        queue.push_back(child);
        ++child->failure->indegree;
      }
    }
    // 二、初始化拓扑排序的无入度节点
    queue.push_back(&this->root);
    while (!queue.empty()) {
      auto n = queue.front();
      queue.pop_front();
      for (auto child: n->children) {
        if (child == nullptr) continue;
        queue.push_back(child);
        if (n->indegree == 0) {
          topological_sort_queue.push_back(n);
        }
      }
    }
    // 三、拓扑排序本体
    while (!topological_sort_queue.empty()) {
      auto n = topological_sort_queue.front();
      topological_sort_queue.pop_front();
      n->failure->times += n->times;
      --n->failure->indegree;
      if (n->failure->indegree == 0)
        topological_sort_queue.push_back(n->failure);
    }
    // 四、返回所有接受状态的字符串出现次数
    queue.push_back(&this->root);
    while (!queue.empty()) {
      auto n = queue.front();
      queue.pop_front();
      for (auto child: n->children) {
        if (child == nullptr) continue;
        queue.push_back(child);
        if (child->accepting != 0 && child->times != 0)
          results.emplace(child->pattern, child->accepting * child->times);
      }
    }
  }

public:
  aho_corasick() = default;
  ~aho_corasick() = default;

  // 构造 Trie 树
  void push(std::string pattern) {
    this->failure_was_built = false;
    auto p = &this->root;
    size_t index = 0;
    for (size_t index = 0; index != pattern.size(); ++index) {
      // assert(is_lower(param[index]));
      size_t k = pattern[index] - 'a';
      if (p->children[k] == nullptr) {
        p->children[k] = new aho_corasick::node();
      }
      p = p->children[k];
    }
    ++p->accepting;
    p->pattern = std::move(pattern);
  }

  // search 方法从母串中搜索自动机中的所有模式串，
  // 返回的 results 表示每个模式串匹配到了多少次。
  // 未命中的模式串不会出现在返回的 map 中。
  std::map<std::string_view, size_t> search(const std::string_view text) {
    this->build_failure();
    aho_corasick::node *p = &this->root;
    for (char c: text) {
      // assert(islower(c));
      size_t k = c - 'a';
      // 若失配则沿着失配链走到根
      while (p != &this->root && p->children[k] == nullptr) p = p->failure;
      if (p->children[k] != nullptr) p = p->children[k];
      // 当前节点匹配到了母串，则将匹配次数增加一次；
      // 当某节点匹配到了的时候，其失效链上所有节点均会匹配，均应更新；
      // 但这里采用惰性更新的方法，在整个母串匹配完后一次性下推标记即可；
      p->times = p->times + 1;
    }
    std::map<std::string_view, size_t> results;
    this->topological_sort_failure(results);
    return results;
  }
};
