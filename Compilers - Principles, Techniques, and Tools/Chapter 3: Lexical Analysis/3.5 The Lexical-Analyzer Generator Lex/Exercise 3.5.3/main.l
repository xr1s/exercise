/* 编写一个 Lex 程序。该程序拷贝一个 C 程序，并将程序中关键字 float 的每个实例替换成 double。
 */

%option noyywrap

%%

([[:alnum:]]|_)+ {
  if (strcmp(yytext, "float") == 0)
    fprintf(yyout, "double");
  else
    ECHO;
}

%%

int main() {
  yylex();
}
