> !
> 像 HTML 或 XML 之类的标记语言不同于传统的程序设计语言，它们要么包含有很多标点符号（标记），如 HTML，要么使用用户自定义的标记集合，如 XML。而且标记还可以带有参数。请指出如何把如下的 HTML 文档
> 
> ```html
> Here is a photo of <B>my house</B>:
> <P><IMG SRC = "house.gif"><BR>
> See <A HREF = "morePix.html">More Pictures</A> if you
> like that one.<P>
> ```
> 
> 划分成适当的词素序列。哪些词素应该有相关联的语法值？应该具有什么值？

绝了，给的 HTML 还有语病。

这道题主要困扰我的地方在于，词法分析和语法分析的界限在哪里？

```
<text "Here is a photo of ">
<start_tag "<B">
<tag_close ">">
<text "my house">
<end_tag "</B">
<tag_close ">">
<text ":\n">

<start_tag "<P">
<tag_close ">">
<start_tag "<IMG">
<attr_name "SRC">
<attr_assign "=">
<attr_value "\"house.gif\"">
<tag_close ">">
<start_tag "<BR">
<tag_close ">">
<text "\n">

<text "See ">
<start_tag "<A">
<attr_name "HREF">
<attr_assign "=">
<attr_value "\"morePix.html\"">
<tag_close ">">
<text "More Pictures">
<end_tag "</A">
<tag_close ">">
<text " if you\n">

<text "liked that one.">
<start_tag "<P">
<tag_close ">">
```
