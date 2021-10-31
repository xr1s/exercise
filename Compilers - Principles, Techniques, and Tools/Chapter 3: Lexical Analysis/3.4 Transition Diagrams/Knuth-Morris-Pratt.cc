#include <string_view>
#include <vector>

std::vector<size_t> failure_function(const std::string_view pattern) {
  std::vector<size_t> failures(pattern.size());
  for (size_t s = 1, t = 0; s != pattern.size(); ++s) {
    while (t != 0 && pattern[t] != pattern[s]) t = failures[t - 1];
    if (pattern[t] == pattern[s]) t = t + 1;
    failures[s] = t;
  }
  return failures;
}

// knuth_morris_pratt 执行在 text 中搜索 pattern
// 并返回所有的匹配位置在母串中的下标。
std::vector<size_t> knuth_morris_pratt(
    const std::string_view text,
    const std::string_view pattern) {
  std::vector<size_t> failures = failure_function(pattern);
  std::vector<size_t> indices;
  for (size_t s = 0, i = 0; i != text.size(); ++i) {
    while (s != 0 && text[i] != pattern[s]) s = failures[s - 1];
    if (text[i] == pattern[s]) s = s + 1;
    if (s == pattern.size()) {
      indices.push_back(i - s + 1);
      s = failures[s - 1];
    }
  }
  return indices;
}
