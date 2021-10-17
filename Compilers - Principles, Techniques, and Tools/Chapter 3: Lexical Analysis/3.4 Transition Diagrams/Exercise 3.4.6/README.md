> 计算得到关键字 *b<sub>1</sub>b<sub>2</sub>...b<sub>n</sub>* 的失效函数之后，我们就可以在 *O(m)* 时间内扫描字符串 *a<sub>1</sub>a<sub>2</sub>...a<sub>m</sub>* 以判断该关键字是否出现在其中。图 3-20 中所展示的算法使关键字沿着被匹配字符串滑动，不断尝试将关键字的下一个字符与被匹配字符串的下一个字符匹配，逐步推进。如果在匹配了 *s* 个字符后无法继续匹配，那么该算法将关键字“向右滑动” *s - f(s)* 个位置，也就是认为只有该关键字的前 *f(s)* 个字符和被匹配字符串匹配。
>
> ```c
> s = 0;
> for (i = 0; i <= m; i++) {
>     while (s > 0 && a[i] != b[s + 1]) s = f(s);
>     if (a[i] == b[s + 1]) s = s + 1;
>     if (s == n) return "yes";
> }
> return "no";
> ```
> <p align="center">图 3-20 KMP 算法在 <i>O(m + n)</i> 时间内检测字符串 <i>a<sub>1</sub>a<sub>2</sub>...a<sub>m</sub></i> 中是否包含单个关键字 <i>b<sub>1</sub>b<sub>2</sub>...b<sub>n</sub></i></p>
>
> 应用 KMP 算法判断关键字 `ababaa` 是否为下面字符串的子串。
>
> 1) `abababaab`  
> 2) `abababbaa`  

首先计算失效函数：

|   *s*  |a|b|a|b|a|a|
|--------|-|-|-|-|-|-|
| *f(s)* |0|0|1|2|3|1|

### 1)

先是 `ababa` 都匹配上了，这时候是这样的：

```
     i
     ↓
abababaab
ababaa
     ↑
    s+1
```

匹配不上了，*s* 开始往回缩。将 *s* 重新赋值为 *s = f(s)*，含义是箭头右移 *s - f(s)* 位，等价于模式串右移 *s - f(s)* 位。含义仍然保持不变，即第 *s* 位及往前的子串均匹配成功了。

如图：

```
     i
     ↓
abababaab
  ababaa
     ↑
    s+1
```

后续两个字符串都能匹配上了，则最终返回了 `"yes"`。

```
       i
       ↓
abababaab
  ababaa
        ↑
       s+1
```

### 2)

同上，首次出现失配在：

```
     i
     ↓
abababbaa
ababaa
     ↑
    s+1
```
此时 *s = 5, f(s) = 3*，则右移 *2* 位。移动后匹配成功，*i* 继续迭代。如图：

```
      i
      ↓
abababbaa
  ababaa
      ↑
     s+1
```

再次失配，此时 *s = 4, f(s) = 2*，右移 *2* 位。（注：注意到模式串已经超出匹配串，其实已经不可能匹配成功）。

```
      i
      ↓
abababbaa
    ababaa
      ↑
     s+1
```

再次失配，此时 *s = 2, f(s) = 0*，右移 *2* 位。

```
      i
      ↓
abababbaa
      ababaa
      ↑
     s+1
```

再次失配，此时 *s = 0*，直接继续迭代 *i*。


```
       i
       ↓
abababbaa
       ababaa
       ↑
      s+1
```

此时匹配上了，但是下一个字符又匹配失败：

```
        i
        ↓
abababbaa
       ababaa
        ↑
       s+1
```

只能再次右移模式串：

```
        i
        ↓
abababbaa
        ababaa
        ↑
       s+1
```

此时虽然匹配上了，但是 *i* 已经到达终点 *m*，迭代结束，匹配失败返回 `"no"`。
