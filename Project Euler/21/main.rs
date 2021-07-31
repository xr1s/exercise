use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use num::{FromPrimitive, ToPrimitive};

// 34561 也是算出来后填回进去的
fn solve<T, R>(range: R) -> T
where
    R: IntoIterator<Item = T>,
    T: Clone + PartialOrd + Sum + FromPrimitive + ToPrimitive,
    for<'a> T: Add<&'a T, Output = T>,
    for<'a> T: Sub<&'a T, Output = T>,
    for<'a> T: Mul<&'a T, Output = T>,
    for<'a> T: Div<&'a T, Output = T>,
{
    let sigma = algorithm::number_theory::sieve::sigma::<T>(34561);
    range
        .into_iter()
        .filter(|d| {
            let a = sigma[d.to_usize().unwrap()].clone() - d;
            a > *d && sigma[a.to_usize().unwrap()] == a + d
        })
        .flat_map(|d| [d.clone(), sigma[d.to_usize().unwrap()].clone() - &d])
        .sum()
}

fn main() {
    println!("{}", solve(1..=10000));
}
