#include <functional>
#include <numeric>
#include <vector>

int findOdd(const std::vector<int> &numbers) {
  return std::reduce(numbers.begin(), numbers.end(), 0, std::bit_xor{});
}

#include <igloo/igloo_alt.h>
using igloo::Assert, igloo::Equals, igloo::TestRunner;

using V = std::vector<int>;
// clang-format off
Describe(FindOdd){
  It(BasicTests){
    Assert::That(findOdd(V{20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5}), Equals(5));
    Assert::That(findOdd(V{1,1,2,-2,5,2,4,4,-1,-2,5}), Equals(-1));
    Assert::That(findOdd(V{20,1,1,2,2,3,3,5,5,4,20,4,5}), Equals(5));
    Assert::That(findOdd(V{10}), Equals(10));
    Assert::That(findOdd(V{1,1,1,1,1,1,10,1,1,1,1}), Equals(10));
  }
};
// clang-format off

int main() {
  igloo::TestRunner::RunAllTests();
}
