> 假设已经计算得到函数 *f* 且它的值储存在一个以 *s* 为下标的数组中，说明图 3-20 中算法的复杂度为 *O(m + n)*。

逻辑和 [练习 3.3.5](../%21%21%20Exercise%203.4.5) 是一样的，首先同理有 `while` 循环最多执行 *n* 次（可以通过势能分析得到），外层的 `for` 循环固定 *m* 次，所以总计 *O(m + n)* 次。

这里也可以得到计算失效函数的复杂度其实应该是 *O(2n) = O(n)*。
