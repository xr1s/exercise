#include <string_view>
#include <vector>

#include <fmt/core.h>

std::string fibonacci_string(size_t n) {
  static std::vector<std::string> mem{"b", "a"};
  if (n > mem.size()) mem.reserve(n);
  for (size_t k = mem.size(); k < n; ++k)
    mem.emplace_back(mem[k - 1] + mem[k - 2]);
  return mem[n - 1];
}

std::vector<size_t> failure_function(std::string_view p) {
  std::vector<size_t> failures(p.size());
  for (size_t s = 0, k = 1; k != p.size(); ++k) {
    while (s != 0 && p[s] != p[k]) s = failures[s - 1];
    if (p[s] == p[k]) s = s + 1;
    failures[k] = s;
  }
  return failures;
}

size_t integer_length(uintmax_t n) {
  return std::to_string(n).size();
}

void println_markdown_failure_function_table(const std::string_view sv) {
  const size_t length = sv.size();
  const auto failures = failure_function(sv);
  fmt::print("\n|  *s* |");
  for (size_t k = 0; k != length; ++k)
    fmt::print("{:^{}}|", sv[k], integer_length(failures[k]) + 2);
  fmt::print("\n|:----:|");
  for (size_t k = 0; k != length; ++k)
    fmt::print(":{:-^{}}:|", "", integer_length(failures[k]));
  fmt::print("\n|*f(s)*|");
  for (size_t k = 0; k != length; ++k) fmt::print(" {} |", failures[k]);
  fmt::print("\n");
}

int main() {
  println_markdown_failure_function_table(fibonacci_string(6));
  println_markdown_failure_function_table(fibonacci_string(7));
}
