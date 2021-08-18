> 根据 3.1.2 节中的讨论， 将下面的 C++ 程序
> 
> ```c++
> float limitedSquare(x) {float x;
>     /* returns x-squared, but never more than 100 */
>     return (x<=-10.0||x>=10.0)?100:x*x;
> }
> ```
> 
> 划分成正确的词素序列。哪些词素应该有相关联的语法值？应该具有什么值？

```
<float>
<identity limitedSquare>
<token left_parenthesis>
<identity x>
<token right_parenthesis>
<token left_brace>
<float>
<identity x>
<token semicolon>

<return>
<token left_parenthesis>
<operator le>
<literal -10.0>
<operator or>
<identity x>
<operator ge>
<literal 10.0>
<token right_parenthesis>
<token question>
<literal 100>
<token colon>
<identity x>
<operator mul>
<identity x>
<token semicolon>

<token right_brace>
```
