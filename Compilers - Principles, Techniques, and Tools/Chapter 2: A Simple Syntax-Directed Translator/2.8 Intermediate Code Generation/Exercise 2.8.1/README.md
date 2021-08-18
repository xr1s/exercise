> C 语言和 Java 语言中的 for 语句具有如下形式：
>
> ```
>     for ( expr1 ; expr2 ; expr3 ) stmt
> ```
> 
> 第一个表达式在循环之前执行，它通常被用来初始化循环下标。第二个表达式是一个测试，它在循环的每次迭代之前进行。如果这个表达式的结果变成 0，就退出循环。循环本身剋被看作语句 `{ stmt expr3; }`。第三个表达式在每一次迭代末尾执行，它通常被用来使循环下标递增。故 for 语句的含义类似于
> ```
>     expr1; while ( expr2 ) {stmt expr3; }
> ```
>
> 仿照图 2-43 中的类 *If*，为 for 语句定义一个类 *For*。

因为是伪代码，就不特别写个 Main.java 了。

```java
class For extends Stmt {
  Expr expr1, expr2, expr3;
  Stmt stmt;

  public For(Expr expr1, Expr expr2, Expr expr3, Stmt stmt) {
   this.expr1 = expr1;
   entr = newlabel();  // 这太怪了，不保存为成员？
   this.expr2 = expr2;
   this.stmt = stmt;
   this.expr3 = expr3;
   exit = newlabel();
  }

  public void gen() {
    this.expr1.rvalue();                                          // expr1
    emit(entr + ":");                                             // entr:
    emit("ifFalse " + this.expr2.rvalue() + " goto " + exit);     // ifFalse expr2 goto exit
    this.stmt.gen();                                              // stmt
    this.expr3.rvalue();                                          // expr3
    emit("goto " + entr);                                         // goto entr
    emit(exit + ":");                                             // exit:
  }
}
```
