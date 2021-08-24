> 对于下列各个语言，查询语句使用手册以确定：  
> （i）  形成各语言的输入字母表的字符集分别是什么（不包括哪些只能出现在字符串或注释中的字符）？  
> （ii） 各语言的数字常量的词法形式是什么？  
> （iii）各语言的标识符的词法形式什么？
> 
> （1）C （2）C++ （3）C# （4）Fortran （5）Java （6）Lisp （7）SQL

这题主要是挺麻烦的，比想象中的要麻烦。  
一是得找各个语言的标准，寻找标准本身就是一件很痛苦的事情，要知道大多标准是收费的；  
二是读标准是很痛苦的，你要不停地在各个定义之间跳转，如果文档不支持跳转，那就更痛苦了；  
三是各个语言支持的字符集不同，比如有的只提到 ASCII，较新的可能支持 Unicode 了，甚至有些是实现定义的，讲清楚太麻烦了。  
因此我的建议是跳过此题，不要浪费时间。

举个例子用来说明复杂性，[Unicode 标准附件 31](https://unicode.org/reports/tr31/) 使用 BNF 定义的标识符形式如：

```
<Identifier> := <Start> <Continue>* (<Medial> <Continue>+)*
```

其中 `<Start>` 默认含有 [`\p{XID_Start}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BXID_Start%7D&g=&i=) 属性；`<Continue>` 默认含有 [`\p{XID_Continue}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&g=&i=) 属性，各语言允许有自己额外的定义；`<Medial>` 是一些语言自定义的连接符号，比如单引号 `'`、减号 `-`、冒号 `:` 等。  

[`\p{ID_Start}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BID_Start%7D&g=&i=) 包含 `\p{L}\p{Nl}\p{Other_ID_Start}` 排除 `\p{Pattern_Syntax}\p{Pattern_White_Space}`；`\p{XID_Start}` 是为了处理 [Unicode 等价字符](https://zh.wikipedia.org/zh-cn/Unicode%E7%AD%89%E5%83%B9%E6%80%A7)，在 `\p{ID_Start}` 的基础上增加了一些经过 [NFKC](https://zh.wikipedia.org/zh-cn/Unicode%E7%AD%89%E5%83%B9%E6%80%A7#%E6%AD%A3%E8%A6%8F%E5%BD%A2%E5%BC%8F) 处理后可能产生的特殊符号；  
[`\p{ID_Continue}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BID_Continue%7D&g=&i=)包含 `\p{ID_Start}\p{Mn}\p{Mc}\p{Nd}\p{Pc}\p{Other_ID_Continue}` 排除 `\p{Pattern_Syntax}\p{Pattern_White_Space}`; `\p{XID_Continue}`、`\p{ID_Continue}` 的关系与 `\p{XID_Start}`、`\p{ID_Start}` 的关系同理。

- [`\p{L}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BPattern_Syntax%7D&g=&i=) 即字母类别的 131241 个码位，包含各语言的大小写各种字母，比如[拉丁字母](https://zh.wikipedia.org/wiki/%E6%8B%89%E4%B8%81%E5%AD%97%E6%AF%8D)、[希腊字母](https://zh.wikipedia.org/wiki/%E5%B8%8C%E8%85%8A%E5%AD%97%E6%AF%8D)、[西里尔字母](https://zh.wikipedia.org/wiki/%E8%A5%BF%E9%87%8C%E5%B0%94%E5%AD%97%E6%AF%8D)等拼音文字，也包含[汉字](https://zh.wikipedia.org/wiki/%E6%B1%89%E5%AD%97)、[日文文字](https://zh.wikipedia.org/wiki/%E6%97%A5%E6%96%87%E6%96%87%E5%AD%97)、[谚文](https://zh.wikipedia.org/wiki/%E6%97%A5%E6%96%87%E6%96%87%E5%AD%97) 等[中日韩统一表意文字](https://zh.wikipedia.org/zh-cn/%E4%B8%AD%E6%97%A5%E9%9F%93%E7%B5%B1%E4%B8%80%E8%A1%A8%E6%84%8F%E6%96%87%E5%AD%97)符号，甚至还有[古埃及圣书体](https://zh.wikipedia.org/wiki/%E5%9C%A3%E4%B9%A6%E4%BD%93)之类的莫名其妙的象形文字；  
- [`\p{Nl}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BNl%7D&g=&i=) 表示数字的字母 236 个码位，有（用于[一种特殊历法](https://en.wikipedia.org/wiki/Runic_calendar)的）[黄金数字](https://en.wikipedia.org/wiki/Golden_number_(time))、[罗马数字](https://zh.wikipedia.org/wiki/%E7%BD%97%E9%A9%AC%E6%95%B0%E5%AD%97)（含[Number Forms 区块](https://en.wikipedia.org/wiki/Number_Forms)中的古罗马数字）、[苏州码子](https://zh.wikipedia.org/zh-cn/%E8%8B%8F%E5%B7%9E%E7%A0%81%E5%AD%90)、[巴姆穆文字](https://en.wikipedia.org/wiki/Bamum_script#Numbers)中的数字、[古希腊字母数字](https://www.compart.com/en/unicode/block/U+10140)、哥特字母数字（𐍁和𐍊）、[楔形文字](https://zh.wikipedia.org/wiki/%E6%A5%94%E5%BD%A2%E6%96%87%E5%AD%97)数字；
- [`\p{Mn}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BMn%7D&g=&i=) 1893 个非空格的修饰符（没找到官方翻译）；
- [`\p{Mc}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BMc%7D&g=&i=) 443 个 空格修饰符；
- [`\p{Nd}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BNd%7D&g=&i=) 650 个数字；
- [`\p{Pc}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BPc%7D&g=&i=) 10 个连接字符，注意 ASCII 中的下划线在这里；
- `\p{Other_ID_Start}` 和 `\p{Other_ID_Continue}` 是特殊维护的、用于在 Unicode 版本间保持该附件向前兼容的小型字符集；
- [`\p{Pattern_Syntax}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BPattern_Syntax%7D&g=&i=) 一般被用于其它语法，它和 ASCII 字符集的交集基本都是标点符号，如 `+-*/()[]<>=` 等等。
- [`\p{Pattern_White_Space}`](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BPattern_White_Space%7D&g=&i=) 一般被用于程序中的空格，它和 ASCII 字符的交集包括控制字符、空格。

几个值得注意的字符：
- 下划线 `_` 在 Unicode 定义的标识符中只存在在 `<Continue>` 中，但是实际上大部分语言都支持其作为 `<Start>`；
- 美元符号 `$` 与下划线类似，在某些语言标准或特殊编译器实现中支持作为 `<Start>` 或 `<Continue>`，如 ECMA Script；
- 在某些语言中，允许单引号 `'`、减号 `-`、冒号 `:` 等字符作为标识符中间的连接字符 `<Medial>`；
- 零宽连字符 `\u200D` 被某些语言允许作为 `<Continue>`，因为它是很多 Emoji 的一部分。

因此后面只讨论 ASCII 定义的字符集，不然麻烦就大了。

#### （1）C

以下摘自 [cppreference.com](https://zh.cppreference.com/w/c/language/translation_phases)。

*源字符集*是包含作为单字节子集的*基本源字符集*的多字节字符集，后者由以下 96 个字符组成：  
- 5 个空白字符（空格、水平制表、垂直制表、换页、换行）  
- 10 个数字字符，从 `0` 到 `9`  
- 52 个字母，从 `A` 到 `Z` 以及从 `a` 到 `z`  
- 29 个标点字符： `_ { } [ ] # ( ) < > % : ; . ? * + - / ^ & | ~ ! = , \ " '`  

为了方便看，这里特地列一下 C 语言不支持出现的可打印符号。

```
$ @ `
```

#### （2）C++

以下翻译自 [C++ 草案](http://eel.is/c++draft/lex.charset)。

*基本源字符集*由 96 个字符组成：空格字符、代表水平制表符、垂直制表符、换页符和换行符的控制字符，以及以下 91 个图形字符：

```
a b c d e f g h i j k l m n o p q r s t u v w x y z
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
0 1 2 3 4 5 6 7 8 9
_ { } [ ] # ( ) < > % : ; . ? * + - / ^ & | ~ ! = , \ " '
```

C++ 的字符集目前和 C 完全一致。

#### C#

完全没写过哈，只是抄了一遍[规范](https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/language-specification/lexical-structure)。

```
a b c d e f g h i j k l m n o p q r s t u v w x y z
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
0 1 2 3 4 5 6 7 8 9
! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ { | } ~
```

空格

```
whitespace
    : '<Any character with Unicode class Zs>'
    | '<Horizontal tab character (U+0009)>'
    | '<Vertical tab character (U+000B)>'
    | '<Form feed character (U+000C)>'
    ;
```

换行

```
new_line
    : '<Carriage return character (U+000D)>'
    | '<Line feed character (U+000A)>'
    | '<Carriage return character (U+000D) followed by line feed character (U+000A)>'
    | '<Next line character (U+0085)>'
    | '<Line separator character (U+2028)>'
    | '<Paragraph separator character (U+2029)>'
    ;
```

不支持的字符

```
`
```

#### Fortran

完全没写过，[标准文档](https://gcc.gnu.org/wiki/GFortranStandards)。

标准里完全找不到空格、回车和其它控制字符能否出现，好像都是实现定义的。  
我理解应该是需要支持的，因为 Fortran 最初 Fixed Format 可是用打孔卡输入的，那不得把 ASCII 全集都用上？

```
a b c d e f g h i j k l m n o p q r s t u v w x y z
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
0 1 2 3 4 5 6 7 8 9
_ ; = ! + " - % * & / ~ \ < ( > ) ? [ ’ ] ` { ^ } | , $ . # : @
```

#### Java

[Java 规范](https://docs.oracle.com/javase/specs/jls/se16/html/jls-3.html)。


```
a b c d e f g h i j k l m n o p q r s t u v w x y z
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
0 1 2 3 4 5 6 7 8 9

! " $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ { | } ~
```

空格及换行

```
WhiteSpace:
the ASCII SP character, also known as "space"
the ASCII HT character, also known as "horizontal tab"
the ASCII FF character, also known as "form feed"
LineTerminator

LineTerminator:
the ASCII LF character, also known as "newline"
the ASCII CR character, also known as "return"
the ASCII CR character followed by the ASCII LF character
InputCharacter:
UnicodeInputCharacter but not CR or LF
```

为了方便看，这里特地列一下 Java 不支持出现的可打印符号。

```
# `
```

#### LISP

因为 LISP 方言实在是太多了，这里就以 [Common LISP 事实标准](https://www.cs.cmu.edu/Groups/AI/html/cltl/cltl2.html)为准（因为实在找不到 ANSI 标准，找到的都是分辨率极低的扫描件，根本不可读）。

```
! " # $ % & ' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? 
@ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \ ] ^ _ 
` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~
```

控制字符（含空格们）

```
#\Backspace  #\Tab  #\Linefeed  #\Page  #\Return  #\Rubout
```

#### SQL

好在有 [BNF](https://github.com/ronsavage/SQL)，这可比[标准](https://en.wikipedia.org/wiki/ISO/IEC_9075)好读多了。  
注意我们不谈 MySQL PostgreSQL 等方言的语法扩展。

```
a b c d e f g h i j k l m n o p q r s t u v w x y z
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
0 1 2 3 4 5 6 7 8 9
! " $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ { | }
```

空格和换行们：

```
— U+0009, Horizontal Tab
— U+000A, Line Feed
— U+000B, Vertical Tabulation
— U+000C, Form Feed
— U+000D, Carriage Return
— U+0020, Space
— U+00A0, No-Break Space
— U+2000, En Quad
— U+2001, Em Quad
— U+2002, En Space
— U+2003, Em Space
— U+2004, Three-Per-Em Space
— U+2005, Four-Per-Em Space
— U+2006, Six-Per-Em Space
— U+2007, Figure Space
— U+2008, Punctuation Space
— U+2009, Thin Space
— U+200A, Hair Space
— U+200B, Zero Width Space
— U+200C, Zero Width Non-Joiner
— U+200D, Zero Width Joiner
— U+200E, Left-To-Right Mark
— U+200F, Right-To-Left Mark
— U+3000, Ideographic Space
— U+2028, Line Separator
— U+2029, Paragraph Separator
— U+FEFF, Zero Width No-Break Space
```

不支持的字符

```
# ~
```
