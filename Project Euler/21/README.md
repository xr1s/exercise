# Amicable numbers

> 找到所有 10000 以下的亲和数对的和.

亲和数就是真因数和各自等于对方的数对, 且要求两者不等.

可以通过欧拉筛 O(n) 求出前 n 个数的除数函数 sigma, 再 O(n) 枚举即可.

## 参考资料
  - [https://oeis.org/A063990](亲和数 OEIS)
  - [https://en.wikipedia.org/wiki/Divisor_function](除数函数)
  - [https://oeis.org/A000203](除数函数 OEIS)
