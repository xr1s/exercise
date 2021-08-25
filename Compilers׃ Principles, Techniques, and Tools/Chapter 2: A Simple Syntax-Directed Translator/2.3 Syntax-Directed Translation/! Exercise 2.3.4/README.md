> 构建一个将罗马数字翻译成整数的语法制导翻译方案。

枚举大法好。


```
one -> {print('0')} ''  | {print('1')} 'I'  | {print('2')} 'II'  | {print('3')} 'III'  | {print('4')} 'IV'
     | {print('5')} 'V' | {print('6')} 'VI' | {print('7')} 'VII' | {print('8')} 'VIII' | {print('9')} 'IX'

ten -> {print('')}  ''  | {print('1')} 'X'  | {print('2')} 'XX'  | {print('3')} 'XXX'  | {print('4')} 'XL'
     | {print('5')} 'L' | {print('6')} 'XL' | {print('7')} 'LXX' | {print('8')} 'LXXX' | {print('9')} 'XC'

hun -> {print('')}  ''  | {print('1')} 'C'  | {print('2')} 'CC'  | {print('3')} 'CCC'  | {print('4')} 'CV'
     | {print('5')} 'D' | {print('6')} 'DC' | {print('7')} 'DCC' | {print('8')} 'DCCC' | {print('9')} 'CX'

tho -> {print('')}  ''  | {print('1')} 'M'  | {print('2')} 'MM'  | {print('3')} 'MMM'

num -> tho | hun | ten | one
```
