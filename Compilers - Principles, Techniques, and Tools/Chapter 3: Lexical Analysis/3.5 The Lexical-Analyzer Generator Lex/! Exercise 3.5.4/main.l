/* 编写一个 Lex 程序。该程序把一个文件改变成为“Pig latin”文。明确地讲，假设该文件是一个用空白符分隔开的单词（即字母串）序列。每当你遇到一个单词时：
 *
 * 1）如果第一个字母是辅音字母，则将它移到单词的结尾，并加上 ay。
 * 2）如果第一个字母是元音字母，则旨在单词的结尾加上 ay。
 *
 * 所有非字母的字符不加处理直接拷贝到输出。
 */

%option noyywrap

%{
int isvowel(int c) {
  switch (c) {
  case 'a': case 'A':
  case 'e': case 'E':
  case 'i': case 'I':
  case 'o': case 'O':
  case 'u': case 'U':
    return 1;
  }
  return 0;
}
%}

%%

[[:alpha:]]+ {
  if (isvowel(*yytext)) {
    fputs(yytext, yyout);
    fputs("ay", yyout);
  } else {
    fputs(yytext + 1, yyout);
    fputc(*yytext, yyout);
    fputs("ay", yyout);
  }
}

%%

int main() {
  yylex();
}
