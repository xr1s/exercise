use std::iter::Sum;
use std::ops::{Mul, Sub};

fn solve<T, R>(range: R) -> T
where
    T: Copy + Mul<Output = T> + Sum + Sub<Output = T>,
    R: Clone + Iterator<Item = T>,
{
    let sum = range.clone().sum::<T>();
    let sqr = range.map(|k| k * k).sum();
    sum * sum - sqr
}

fn main() {
    println!("{}", solve(1..=100));
}
