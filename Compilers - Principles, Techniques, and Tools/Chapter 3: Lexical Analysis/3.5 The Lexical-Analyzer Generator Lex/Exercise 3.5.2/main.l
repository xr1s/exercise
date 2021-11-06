/* 编写一个 Lex 程序，该程序拷贝一个文件，并将文件中的每个非空的空白符序列替换为单个空格。
 */

%option noyywrap

%%

[[:space:]]+ { fputc(' ', yyout); }

%%

int main() {
  yylex();
}
