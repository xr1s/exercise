/* 在 SQL 中，，关键字和标识符都是大小写不敏感的。编写一个 Lex 程序，该程序识别（大小写字母任意组合的）关键字 SELECT、FROM 和 WHERE 以及词法单元 ID。考虑到这个练习的目的，你可以把 ID 看成是任何以一个字母开头、由字母和数位组成的字符串。你不必将标识符存放到一个符号表中，但需要指出这里的“install”函数与图 3-23 中用于描述大小写敏感标识符的函数有何不同。
 */

%{

#include <ctype.h>
#include <stdio.h>

%}

%option noyywrap

WHILE     (?i:while)
FROM      (?i:from)
SELECT    (?i:select)
ID        ([[:alpha:]]|_)([[:alnum:]]|_)*

%%

{WHILE}   { fputs("<keyword WHILE>", yyout); }
{FROM}    { fputs("<keyword FROM>", yyout); }
{SELECT}  { fputs("<keyword SELECT>", yyout); }
{ID}      {
  fputs("<identity ", yyout);
  for (size_t k = 0; k != yyleng; ++k)
    fputc(toupper(yytext[k]), yyout);
  fputc('>', yyout);
}

%%

int main() {
  yylex();
}
