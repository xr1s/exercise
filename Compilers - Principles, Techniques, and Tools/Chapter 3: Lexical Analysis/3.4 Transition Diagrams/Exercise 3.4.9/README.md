> *Fibonacci* 字符串的定义如下：
>
> 1) *s<sub>1</sub>* = `b`。  
> 2) *s<sub>2</sub>* = `a`。  
> 3) 当 *k &gt; 2* 时，*s<sub>k</sub> = s<sub>k - 1</sub>s<sub>k - 2</sub>*。  
>
> 例如，*s<sub>3</sub> =* `ab`，*s<sub>4</sub> =* `aba`，*s<sub>5</sub> =* `abaab`。
>
> 1）*s<sub>n</sub>* 的长度是多少？  
> 2）构造 *s<sub>6</sub>* 的失效函数。  
> 3）构造 *s<sub>7</sub>* 的失效函数。  
> !! 4）说明任何 *s<sub>n</sub>* 的失效函数都可以被表示为：*f(1) = f(2) = 0*，且对于 *2 &lt; j &le; |s<sub>n</sub>|, f(j) = j - |s<sub>k - 1</sub>|*，其中 *k* 是使得 *|s<sub>k</sub>| &le; j + 1* 的最大的整数。  
> !! 5）在 KMP 算法中，当我们试图确定关键字 *s<sub>k</sub>* 是否出现在字符串 *s<sub>k + 1</sub>* 中时，最多会连续多少次调用失效函数？  

为了方便做题（观察规律），写了一个计算失效函数的 rust 程序（依赖外部库 once\_cell），也可以作为 0 开始下标字符串的失败函数怎么求的参考。

#### 1）

显然，这是斐波那契数列。*k &gt; 2* 时有 *|s<sub>k</sub>| = |s<sub>k - 1</sub>s<sub>k - 2</sub>| = |s<sub>k - 1</sub>| + |s<sub>k - 2</sub>|*，而 *|s<sub>1</sub>| = |s<sub>2</sub>| = 1*。这恰好是斐波那契数列的递推式。

方便起见，下文斐波那契数列的第 *n* 项用 *F<sub>n</sub>* 表示。

这意味着 *|s<sub>n</sub>| = F<sub>n</sub>*。

#### 2）

跑一下程序就有了。

|  *s* | a | b | a | a | b | a | b | a |
|:----:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|*f(s)*| 0 | 0 | 1 | 1 | 2 | 3 | 2 | 3 |

#### 3）

|  *s* | a | b | a | a | b | a | b | a | a | b | a | a | b |
|:----:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|*f(s)*| 0 | 0 | 1 | 1 | 2 | 3 | 2 | 3 | 4 | 5 | 6 | 4 | 5 |

#### 4）

上面已经证明了 *|s<sub>n</sub>| = F<sub>n</sub>*，后面都用 *F<sub>n</sub>* 表示长度。

当 *n = 3* 和 *n = 4* 时，失效函数显然是满足条件的。后续只讨论 *n &gt; 4* 的情况。

当 *n &gt; 4* 时，我们可以将 *s<sub>n</sub>* 展开为： *s<sub>n</sub> = s<sub>n - 1</sub>s<sub>n - 2</sub> = s<sub>n - 2</sub>s<sub>n - 3</sub>s<sub>n - 2</sub>*。  
这里显然最长前后缀至少是 *s<sub>n - 2</sub>*，那么前后缀中相对应的字符距离就是 *F<sub>n - 2</sub> + F<sub>n - 3</sub> = F<sub>n - 1</sub>*。  

先考虑边界条件 *j = F<sub>n</sub>* 或 *j = F<sub>n</sub> - 1* 的时候，  
根据公式 *s<sub>n</sub> = s<sub>n - 2</sub>s<sub>n - 3</sub>s<sub>n - 2</sub>*，显然有当 *j = F<sub>n</sub>* 和 *j = F<sub>n</sub> - 1* 时，分别有 *f(F<sub>n</sub>) &ge; F<sub>n - 2</sub>* 和 *f(F<sub>n</sub> - 1) &ge; F<sub>n - 2</sub> - 1*。  
**（待证明：*j = F<sub>n</sub>* 或 *F<sub>n</sub> - 1* 时，*f(F<sub>n</sub>) = F<sub>n - 2</sub>* 及 *f(F<sub>n</sub> - 1) = F<sub>n - 2</sub> - 1*，注意等号）**  
当 *j = F<sub>n</sub>* 时，有 *f(j) = F<sub>n - 2</sub> = F<sub>n</sub> - F<sub>n - 1</sub> = j - F<sub>n - 1</sub>*，由于使得 *F<sub>k</sub> ≤ n + 1* 最大的 *k* 是 *n*，所以 *f(j)* 满足公式。  
当 *j = F<sub>n</sub> - 1* 时，有 *f(j) = F<sub>n - 2</sub> - 1 = (F<sub>n</sub> - 1) - F<sub>n - 1</sub> = j - F<sub>n - 1</sub>*，由于使得 *F<sub>k</sub> ≤ n + 1* 最大的 *k* 是 *n*，所以 *f(j)* 满足公式。  

Fibonacci 字符串有一个有趣的性质，就是删去最后两个字符串，它们本身是回文的。  
证明会放到最后，我们可以利用这个性质得到失效函数的通项公式。

因为对于 *n &gt; 2* 而言，*s<sub>n</sub> = s<sub>n - 1</sub>s<sub>n - 2</sub>*，那么 *s<sub>n</sub>* 的前 *F<sub>n - 1</sub>* 个失效函数值是和 *s<sub>n - 1</sub>* 是一样的，我们只需要求最后 *s<sub>n - 2</sub>* 部分的失效函数即可。  
又因为 *s<sub>n</sub> = s<sub>n - 2</sub>s<sub>n - 3</sub>s<sub>n - 2</sub>*，且除去最后两个字符后本身是回文串，就有 *s<sub>n - 2</sub>s<sub>n - 3</sub>[:-2] = s<sub>n - 3</sub>s<sub>n - 2</sub>[:-2]*。  
那么很明显，当 *F<sub>n - 1</sub> &lt; j &le; F<sub>n</sub> - 2* 时，子串，有 *f(j) &ge; j - F<sub>n - 2</sub>*。  
又因为根据定义 *f* 是最多以 *1* 递增的，且根据上文有 *f(F<sub>n - 1</sub>) = F<sub>n - 3</sub> = F<sub>n - 1</sub> - F<sub>n - 2</sub>*，  
那么当 *F<sub>n - 1</sub>&lt; j &le; F<sub>n</sub> - 2* 时，*f(j) &le; f(F<sub>n - 1</sub>) + j - F<sub>n - 1</sub> = j - F<sub>n - 2</sub>*。
所以 *f(j) = j - F<sub>n - 2</sub>*。

下面证明对于 *n &gt; 2* 的 *s<sub>n</sub>*，删去最后两个字符后是回文串。（归纳法）  
首先 *s<sub>3</sub>* 和 *s<sub>4</sub>* 是显然符合的。  
另一个规律，当 *n* 为奇数时，最后两个字符永远是 `ab`，当 *n* 为偶数时，最后两个字符永远是 `ba`。  
这条规律证明很简单，因为 *s<sub>n</sub> = s<sub>n - 1</sub>s<sub>n - 2</sub>*，所以后缀是和 *s<sub>n - 2</sub>* 一样的，那么就永远和 *s<sub>3</sub>*、*s<sub>4</sub>* 一样。  
对于 *s<sub>n</sub> = s<sub>n - 2</sub>s<sub>n - 3</sub>s<sub>n - 2</sub>*，根据归纳  
前后 *F<sub>n - 2</sub> - 2* 个字符都是回文串，即前后两个 *s<sub>n - 2</sub>* 除去最后两个字符部分；  
中间第 *F<sub>n - 2</sub> + 1* 开始到 *F<sub>n - 1</sub> - 2* 为止也是回文，即 *s<sub>n - 3</sub>* 除去最后两个字符部；  
那么还有第一个 *s<sub>n - 2</sub>* 的后缀和 *s<sub>n - 3</sub>* 的后缀，它们必然一个是 `ab` 一个是 `ba`；  
这就可以证明它是回文串，因为整串前后两个 *s<sub>n - 2</sub>* 是回文且相等，中间 *s<sub>n - 3</sub>* 也是回文，剩下对应位置的两小块分别是 `ab` 和 `ba`，  
所以就有 *s<sub>n</sub>* 的翻转等于 *s<sub>n</sub>* 本身，这就是回文串的充要条件。

#### 5）

这不是显然是 0 吗，我不理解。

因为 *s<sub>k</sub>* 肯定是 *s<sub>k + 1</sub>* 的前缀呀，不会出现失配的情况。
