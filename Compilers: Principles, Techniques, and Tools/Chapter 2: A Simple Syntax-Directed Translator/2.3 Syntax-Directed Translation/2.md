> 构建一个语法制导翻译方案，该方案将算术表达式从后缀表示方式翻译到中缀表示方式。给出输入 95-2* 和 952*- 的注释分析树。

```
expr -> expr {print(add)} expr add
      | term {print(mul)} term mul
      | {print(d)} d

term -> {print('(')} expr {print(add)} expr add {print(')'}
      | term {print(mul)} term mul
      | {print(d)} d

add -> '+' | '-'
mul -> '*' | '/'
d -> '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
```

`95-2*`

<table>
  <tr>
    <td align="center" valign="top" colspan="12" rowspan="1">term</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="8" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(mul) }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="1">mul</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="3">{ print('(') }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(add) }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">add</td>
    <td align="center" valign="top" colspan="1" rowspan="3">{ print(')') }</td>
    <td align="center" valign="top" colspan="1" rowspan="3">{ print('*') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(d) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">d</td>
    <td align="center" valign="top" colspan="1" rowspan="3">'*'</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(d) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">d</td>
    <td align="center" valign="top" colspan="1" rowspan="2">{ print('-') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(d) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">d</td>
    <td align="center" valign="top" colspan="1" rowspan="2">'-'</td>
    <td align="center" valign="top" colspan="1" rowspan="2">{ print('2') }</td>
    <td align="center" valign="top" colspan="1" rowspan="2">'2'</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('9') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'9'</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('5') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'5'</td>
  </tr>
  <tr>
    <th>(</th>
    <th>9</th>
    <th></th>
    <th>-</th>
    <th>5</th>
    <th></th>
    <th></th>
    <th>)</th>
    <th>*</th>
    <th>2</th>
    <th></th>
    <th></th>
  </tr>
</table>

`952*-`

<table>
  <tr>
    <td align="center" valign="top" colspan="10" rowspan="1">expr</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="2" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(add) }</td>
    <td align="center" valign="top" colspan="6" rowspan="1">expr</td>
    <td align="center" valign="top" colspan="1" rowspan="1">add</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(d) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">d</td>
    <td align="center" valign="top" colspan="1" rowspan="3">{ print('-') }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(mul) }</td>
    <td align="center" valign="top" colspan="2" rowspan="1">term</td>
    <td align="center" valign="top" colspan="1" rowspan="1">mul</td>
    <td align="center" valign="top" colspan="1" rowspan="3">'-'</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="2">{ print('9') }</td>
    <td align="center" valign="top" colspan="1" rowspan="2">'9'</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(d) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">d</td>
    <td align="center" valign="top" colspan="1" rowspan="2">{ print('*') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print(d) }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">d</td>
    <td align="center" valign="top" colspan="1" rowspan="2">'*'</td>
  </tr>
  <tr>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('5') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'5'</td>
    <td align="center" valign="top" colspan="1" rowspan="1">{ print('2') }</td>
    <td align="center" valign="top" colspan="1" rowspan="1">'2'</td>
  </tr>
  <tr>
   <th>9</th>
   <th></th>
   <th>-</th>
   <th>5</th>
   <th></th>
   <th>*</th>
   <th>2</th>
   <th></th>
   <th></th>
   <th></th>
  </tr>
</table>
