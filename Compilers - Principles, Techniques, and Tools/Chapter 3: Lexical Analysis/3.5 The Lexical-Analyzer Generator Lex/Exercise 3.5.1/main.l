/* 描述如何对图 3-23 中的 Lex 程序做如下修改：
 *
 * 1）增加关键字 while。
 * 2）将比较运算符转变成 C 语言中的同类运算符。
 * 3）允许把下划线当作一个附加的字母。
 * ! 4）增加一个新的具有此法单元 STRING 的模式。该模式由一个双引号（"）、任意字符串以及结尾处的一个双引号组成。但是，如果一个双引号出现在上述串中，那么它的前面必须加上一个反斜线（\）进行转义处理，因此在该字符串中的反斜线将用双反斜线表示。这个词法单元的词法值是去掉了双引号的字符串，并且其中用于转义的反斜线已经被删除。识别得到的字符串将北存放到一个字符串表中去。
 */

%option noyywrap

%{
  /* definitions of manifest constants */
#define LT 3001
#define LE 3002
#define EQ 3003
#define NE 3004
#define GT 3005

#define IF 1001
#define THEN 1002
#define ELSE 1003
#define WHILE 1004

#define ID 2001
#define NUMBER 2002
#define RELOP 2003
#define STRING 2004

int yylval;
%}

/* regular definition */
string    \"([^\\\"]|\\\"|\\\\)*\"
delim     [ \t\n]
ws        {delim}+
letter    [A-Za-z_]
digit     [0-9]
id        {letter}({letter}|{digit})*
number    {digit}+(\.{digit}+)?(E[+-]?{digit}+)?

%%

{string}  {yylval = (int) installString(); return(STRING);}
{ws}      { /* no action and no return */ }
if        {return(IF);}
then      {return(THEN);}
else      {return(ELSE);}
while     {return(WHILE);}
{id}      {yylval = (int) installID(); return(ID);}
{number}  {yylval = (int) installNum(); return(NUMBER);}
"<"       {yylval = LT; return(RELOP);}
"<="      {yylval = LE; return(RELOP);}
"=="      {yylval = EQ; return(RELOP);}
"!="      {yylval = NE; return(RELOP);}
">"       {yylval = LT; return(RELOP);}
">="      {yylval = NE; return(RELOP);}
<<EOF>>   { return EOF; }
.         /* do not ECHO */

%%

int installID() {/* function to install the lexeme, whose
                    first character is pointed to by yytext,
                    and whose length is yyleng, into the
                    symbol table and return a pointer
                    thereto */
  return 0;
}

int installNum() {/* similar to installID, but puts numer-
                    ical constants into a separate table */
  return 0;
}

int installString() {
  /* Actions are free to modify yytext except for lengthening it (adding characters to its
   * end–these will overwrite later characters in the input stream). This however does not apply
   * when using %array (see Chapter 7 [Matching], page 14). In that case, yytext may be freely
   * modified in any way.
   */
  size_t j = 0;
  for (size_t i = 0; i != yyleng; ++i) {
    if (i == 0 || i == yyleng - 1) continue;
    if (yytext[i] == '\\') ++i;
    yytext[j++] = yytext[i];
  }
  yytext[yyleng = j] = '\0';
  return 0;
}

int main() {
  for (int r = yylex(); r != EOF; r = yylex())
    printf("return: %d, yytext: %s\n", r, yytext);
}
