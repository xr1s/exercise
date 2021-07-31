# Special Pythagorean triplet

> 求唯一一个满足 a² + b² = c² 且 a + b + c = 1000 的 (a, b, c) 的乘积 a * b * c.

枚举 (a, b) 就够了.

简单推导一下有以下结论, 但是实际上没啥特殊的优化可用.

a² + b² = c²  
a + b = n - c  
a² + b² + 2ab = n² + c² - 2nc  
2ab + 2nc = n²  
2ab + 2n² - 2na - 2nb = n²  
2ab + n² = 2na + 2nb  
