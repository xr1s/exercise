use std::ops::RangeBounds;

use num::BigInt;

fn solve<T, R>(range: R) -> i64
where
    BigInt: From<T>,
    R: Iterator<Item = T> + RangeBounds<T>,
{
    range
        .map(BigInt::from)
        .product::<BigInt>()
        .to_string()
        .as_bytes()
        .iter()
        .map(|byte| byte - b'0')
        .map(|byte| byte as i64)
        .sum()
}

fn main() {
    println!("{}", solve(1..=100));
}
