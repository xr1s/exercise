> 对图 1-13a 中的块结构 C 代码，指出赋值给 *w*、*x*、*y*、*z* 的值。
>
> ```c
> int w, x, y, z;
> int i = 4; int j = 5;
> {   int j = 7;
>     i = 6;
>     w = i + j;
> }
> x = i + j;
> {   int i = 8;
>     y = i + j;
> }
> z = i + j;
> ```

w = 13  
x = 11  
y = 13  
z = 11  
