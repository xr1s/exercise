> 练习 2.2.2 中有哪些文法距有二义性？

1）始终只有一个生成模式，不可能有二义性；  

2）无二义性，前缀表达式和后缀表达式都无二义性；  

3）`()()` 就是一个有歧义的串  
可以是 `S -> S(S)S -> ()(ε)ε`  
也可以 `S -> S(S)S -> ε(ε)()`  

4）`abab` 就是一个有歧义的串  
可以是 `S -> aSbS -> a ε b aSbS -> a ε b a ε b ε`  
也可以 `S -> aSbS -> a bSaS b ε -> a b ε a ε b ε`  

5）`aaa` 就是一个有歧义的串  
可以是 `S -> SS -> a SS -> a a a`  
也可以 `S -> SS -> SS a -> a a a`  
