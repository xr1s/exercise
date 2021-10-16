> 对 *s* 进行归纳，证明图 3-19 的算法正确地计算出了失效函数。

下面下标也都从 1 开始，跟随书本。

根据定义，显然 *f(1)* 只能为 0。另外根据赋值逻辑可以得出，*t* 即为前一个字符的失败函数值。

对图中的 *s*，当 *s >= 1* 时，分 *t = 0* 或 *t ≠ 0* 两种情况。

假设所有小于 s 的失败函数值都是满足条件的最大值：

* 当 *t = 0* 时，说明 *b<sub>1</sub>b<sub>2</sub>...b<sub>s</sub>* 中没有相等的对应前缀后缀。也就是说，不存在一个小于 *s* 的正整数 *k* 使得子串 *b<sub>1</sub>b<sub>2</sub>...b<sub>k</sub>* 和 *b<sub>s - k + 1</sub>b<sub>s - k + 2</sub>...b<sub>s</sub>* 相等。  
因此根据定义，只需要 *b<sub>s + 1</sub> = b<sub>1</sub>* 即满足长度为 1 的前、后缀相等，就可以使得 *f(s + 1) = 1*；若不满足，则为 *0*。

* 当 *t ≠ 0* 则说明 *b<sub>1</sub>b<sub>2</sub>...b<sub>t</sub>* 和 *b<sub>s - t + 1</sub>b<sub>s - t + 2</sub>...b<sub>s</sub>* 相等，此时它们互为 *b<sub>1</sub>b<sub>2</sub>...b<sub>s</sub>* 的前后缀。  
  * 当 *b<sub>t + 1</sub> = b<sub>s + 1</sub>* 时，可得 *b<sub>1</sub>b<sub>2</sub>...b<sub>t</sub>b<sub>t + 1</sub>* 和 *b<sub>s - t + 1</sub>b<sub>s - t + 2</sub>...b<sub>s</sub>b<sub>s + 1</sub>* 相等，它们互为 *b<sub>1</sub>b<sub>2</sub>...b<sub>s</sub>b<sub>s + 1</sub>* 的前后缀，所以 *f(s + 1) = f(s) + 1 = t + 1*。
  * 当 *b<sub>t + 1</sub> ≠ b<sub>s + 1</sub>* 时
    1. 当 *f(t) = 0* ，说明没有能匹配的前后缀了，这时候直接判断 *b<sub>1</sub>* 和 *b<sub>s + 1</sub>* 是否相等，相等则作为新的前后缀，*f(s + 1) = 1*，否则为 *0*；
    2. 当 *f(t) ≠ 0*，我们仍然有 *b<sub>1</sub>b<sub>2</sub>...b<sub>f(t)</sub>* 和 *b<sub>t - f(t) + 1</sub>b<sub>t - f(t) + 2</sub>...b<sub>t</sub>* 相等，因为根据归纳小于 *s* 的失败函数都正确计算了。  
      又因为 *f(s)* 的原因我们有 *b<sub>1</sub>b<sub>2</sub>...b<sub>t</sub>* 和 *b<sub>s - t + 1</sub>b<sub>s - t + 2</sub>...b<sub>s</sub>* 相等，  
      就可以得到 *b<sub>1</sub>b<sub>2</sub>...b<sub>f(t)</sub>* 和 *b<sub>s - f(t) + 1</sub>b<sub>s - f(t) + 2</sub>...b<sub>s</sub>* 相等，  
      就可以继续通过判断 *b<sub>f(t) + 1</sub>* 和 *b<sub>s + 1</sub>* 是否相等来确定 *f(s + 1)* 的值，若相等则为 *f(t) + 1 = f(f(s)) + 1*。  
      若 *b<sub>f(t) + 1</sub> ≠ b<sub>s + 1</sub>* 则可以同理继续判断 *f(f(t))*，该过程可以一直继续直到 *f<sup>n</sup>(t) = 0* 为止，这时候回到 a. 分支（即从零开始）。  
      **证明是最大值：** 由于 *t = f(s)* 已经是 *s* 内满足条件的最大值，次大的肯定在 *b<sub>1</sub>b<sub>2</sub>...b<sub>t</sub>* 以内，则根据归纳， *f(t) = f(f(s))* 即为 *t* 内满足条件的最大值、*s* 内满足条件的次大值。由于 *f(s) + 1* 不满足条件，则 *f(t) + 1* 即为 *f(s + 1)* 且为最大值。  
      该推论对于 *f(t)* 以及 *f<sup>n</sup>(t)* 内也同理，即 *f<sup>n</sup>(t)* 始终是 *f<sup>n - 1</sup>(t)* 内满足条件的最大值，是 f<sup>n - 2</sup>(t) 内满足条件的次大值，依此类推，直到 *f<sup>0</sup>(t) = f(s) = t*。  
      那么当前序都不满足的时候，第一个使得 *b<sub>f<sup>n</sup>(t) + 1</sub> = b<sub>s + 1</sub>* 的 *f<sup>n</sup>(t) + 1* 即为最大值。
