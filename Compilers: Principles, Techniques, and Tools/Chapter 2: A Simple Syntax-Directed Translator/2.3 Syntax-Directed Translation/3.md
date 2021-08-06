> 构建一个将整数翻译成罗马数字的语法制导翻译方案。

枚举大法好。

```
one -> {print('')}  '0' | {print('I')}  '1' | {print('II')}  '2' | {print('III')}  '3' | {print('IV')} '4'
     | {print('V')} '5' | {print('VI')} '6' | {print('VII')} '7' | {print('VIII')} '8' | {print('IX')} '9'

ten -> {print('')}  ''  | {print('X')}  '1' | {print('XX')}  '2' | {print('XXX')}  '3' | {print('XL')} '4'
     | {print('L')} '5' | {print('LX')} '6' | {print('LXX')} '7' | {print('LXXX')} '8' | {print('XC')} '9'

hun -> {print('')}  '0' | {print('C')}  '1' | {print('CC')}  '2' | {print('CCC')}  '3' | {print('CV')} '4'
     | {print('D')} '5' | {print('DC')} '6' | {print('DCC')} '7' | {print('DCCC')} '8' | {print('CX')} '9'

tho -> {print('')}  ''  | {print('M')}  '1' | {print('MM')}  '2' | {print('MMM')}  '3'

num -> tho | hun | ten | one

```
