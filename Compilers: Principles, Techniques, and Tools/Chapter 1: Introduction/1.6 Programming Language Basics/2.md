> 对图 1-13b 中的代码重复练习 1.6.1。
>
> ```c
> int w, x, y, z;
> int i = 3; int j = 4;
> {   int i = 5;
>     w = i + j;
> }
> x = i + j;
> {   int j = 6;
>     i = 7;
>     y = i + j;
> }
> z = i + j;
> ```

w = 9  
x = 7  
y = 13  
z = 11  
