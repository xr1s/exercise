> 正则表达式 `r{m,n}` 和模式 `r` 的 `m` 次到 `n` 次重复出现相匹配。例如，`a{1,5}` 和由 1 ~ 5 个 a 组成的串匹配。试证明：对于每一个包含这种形式的重复运算符的正则表达式，都存在一个等价的不包含重复运算符的正则表达式。

这不是展开就好了吗？把 `r` 重复 `m` 次展开到重复 `n` 次，然后或起来，然后括号括住。
