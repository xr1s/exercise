> 试说明在一个长度为 n 的字符串中，分别由多少个
> 1）前缀
> 2）后缀
> 3）真前缀
> ！4）子串
> ！5）子序列

1）n + 1

2）n + 1

3）n - 1

4） 当组成字符串的字符各不相同时  
考虑子串在原串中左右边界的位置  
共 C(n + 1, 2) 种组合  
因此子串有 n (n + 1) / 2 + 1 种

当原串中存在相同字符时，就要视情况讨论了

5）当组成字符串的字符各不相同时  
考虑原串中的每个字符是否出现在子序列中  
对于每个字符都有两种可能  
因此子序列共有 2ⁿ 种

当原串中存在相同字符时，就要视情况讨论了