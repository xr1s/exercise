> 下面的 C 代码的打印结果是什么？
>
> ```c
> #define a (x+1)
> int x = 2;
> void b() { x = a; printf("%d\n", x); }
> void c() { int x = 1; printf("%d\n", a); }
> void main() { b(); c(); }
> ```

首先我们展开宏

```c
int x = 2;
void b() { x = (x+1); printf("%d\n", x); }
void c() { int x = 1; printf("%d\n", (x+1)); }
void main() { b(); c(); }
```

所以输出

```
3
2
```
