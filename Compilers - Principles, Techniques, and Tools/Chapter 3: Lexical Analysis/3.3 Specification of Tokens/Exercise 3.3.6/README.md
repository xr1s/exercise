> 为下列字符集合写出对应的字符类。
> 
> 1）英文字母的前 10 个字母（从 a~j），包括大写和小写。  
> 2）所有小写的辅音字母的集合。  
> 3）十六进制中的“数位”（对大于 9 的数位，自己决定大写或小写）。  
> 4）可以出现在一个合法的英语句子后面的字符集（比如感叹号）。
> 
> 从下面开始直到练习 3.3.10（含）讨论了来自 Lex 的正则表达式的扩展表示方法（我们将在 3.5 节中讨论这个词法分析分析器生成工具）。这些扩展表示方法再图 3-8 中列出。
> 
> | 表达式   | 匹配                                    | 例子      |
> |:--------:|:----------------------------------------|:---------:|
> | `c`      | 非单个运算符字符 `c`                    | `a`       |
> | `\c`     | 字符 `c` 的字面值                       | `\*`      |
> | `"s"`    | 串 `s` 的字面值                         | `"**"`    |
> | `.`      | 除换行符以外的任何字符                  | `a.*b`    |
> | `^`      | 一行的开始                              | `^abc`    |
> | `$`      | 行的结尾                                | `abc$`    |
> | `[s]`    | 字符串 `s` 中的任何一个字符             | `[abc]`   |
> | `[^s]`   | 不在串 `s` 中的任何一个字符             | `[^abc]`  |
> | `r*`     | 和 `r` 匹配的零个或多个串链接成的串     | `a*`      |
> | `r+`     | 和 `r` 匹配的一个或多个串链接成的串     | `a+`      |
> | `r?`     | 零个或一个 `r`                          | `a?`      |
> | `r{m,n}` | 最少 `m` 个，最多 `n` 个 `r` 的重复出现 | `a{1,5}`  |
> | `r₁r₂`   | `r₁` 后加上 `r₂`                        | `ab`      |
> | `r₁|r₂`  | `r₁` 或 `r₂`                            | `a|b`     |
> | `(r)`    | 与 `r` 相同                             | `(a|b)`   |
> | `r₁/r₂`  | 后面跟有 `r₂` 时的 `r₁`                 | `abc/123` |

### 1）英文字母的前 10 个字母（从 a~j），包括大写和小写。

`[a-jA-J]`

### 2）所有小写的辅音字母的集合。

`[bcdfghjklmnpqrstvwxzy]`

### 3）十六进制中的“数位”（对大于 9 的数位，自己决定大写或小写）。  

`[0-9A-Fa-f]`

### 4）可以出现在一个合法的英语句子后面的字符集（比如感叹号）。

`[.!?]`
