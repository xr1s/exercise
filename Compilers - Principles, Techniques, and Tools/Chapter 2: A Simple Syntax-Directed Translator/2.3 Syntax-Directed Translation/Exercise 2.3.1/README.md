> 构建一个语法制导方案，该方案把算术表达式从中缀表示方式翻译成运算符在运算分量之前的前缀表示方式。例如，-xy 是表达式 x-y 的前缀表示法。给出输入 9-5+2 和 9-5*2 的注释分析树。

```
expr -> {print(add)} expr add term | term
term -> {print(mul)} term mul fact | fact
fact -> {print(digit)} digit | '(' expr ')'
add -> '+' | '-'
mul -> '*' | '/'
digit -> '0' | '1' | '2' | '3' | '4'
       | '5' | '6' | '7' | '8' | '9'
```

画图比做题还难

`9-5+2`

<table>
  <tr>
    <td align="center" valign="top" colspan="10">expr</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(add) }</td>
    <td align="center" valign="top" colspan="6" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">add</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="5">{ print('+') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(add) }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">add</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="5">'+'</td>
    <td align="center" valign="top" colspan="2" rowspan="1">fact</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="4">{ print('-') }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="4">'-'</td>
    <td align="center" valign="top" colspan="2" rowspan="1">fact</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(digit) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">digit</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="2" rowspan="1">fact</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(digit) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">digit</td>
    <td align="center" valign="top" colspan="1" rowspan="3">{ print('2') }</td>
    <td align="center" valign="top" colspan="1" rowspan="3">'2'</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(digit) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">digit</td>
    <td align="center" valign="top" colspan="1" rowspan="2">{ print('5') }</td>
    <td align="center" valign="top" colspan="1" rowspan="2">'5'</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('9') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'9'</td>
  </tr>
  <tr>
    <th>+</th>
    <th>-</th>
    <th>9</th>
    <th></th>
    <th></th>
    <th>5</th>
    <th></th>
    <th></th>
    <th>2</th>
    <th></th>
  </tr>
</table>

`9-5*2`

<table>
  <tr>
    <td align="center" valign="top" colspan="10" rowspan="1">expr</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(add) }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">add</td>
    <td align="center" valign="top" colspan="6" rowspan="1">term</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="4">{ print('-') }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="4">'-'</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(mul) }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="1">mul</td>
    <td align="center" valign="top" colspan="2" rowspan="1">fact</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="2" rowspan="1">fact</td>
    <td align="center" valign="top" colspan="1" rowspan="3">{ print('*') }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">fact</td>
    <td align="center" valign="top" colspan="1" rowspan="3">'*'</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(digit) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">digit</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(digit) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">digit</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(digit) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">digit</td>
    <td align="center" valign="top" colspan="1" rowspan="2">{ print('9') }</td>
    <td align="center" valign="top" colspan="1" rowspan="2">'9'
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('9') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'9'</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('5') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'5'</td>
  </tr>
  <tr>
    <th>-</th>
    <th>9</th>
    <th></th>
    <th></th>
    <th>*</th>
    <th>5</th>
    <th></th>
    <th></th>
    <th>2</th>
    <th></th>
  </tr>
</table>
