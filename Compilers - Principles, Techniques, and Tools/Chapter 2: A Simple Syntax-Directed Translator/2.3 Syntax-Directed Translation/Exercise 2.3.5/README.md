> 构建一个讲后缀算术表达式翻译成等价的前缀算术表达式的语法制导翻译方案。

```
expr -> {print(op)} expr expr op | {print(digit)} digit
```
