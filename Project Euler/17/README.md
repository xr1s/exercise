# Number letter counts

> 1 到 1000 (含) 的整数英文表示的字母数.
> 需要注意的是几百几十几中间是含 and 的.

主要难点在数字转英文, 我才懒得写.

Rust 用了 english-numbers 包.
Haskell 用了 ordinal 包. 有个语言更丰富的 numerals, 之所以不用是因为提供的英文转换几百几十几没有 and.
Go 用了 github.com/divan/num2words 包. 有个语言更丰富的 moul.io/number-to-word, 之所以不用它是因为提供的英文转换几百几十几没有 and.
