use std::{
    iter::Sum,
    ops::{AddAssign, Rem},
};

fn solve<R, D, T, S>(range: R, ds: D) -> T
where
    R: IntoIterator<Item = T>,
    D: Copy + IntoIterator<Item = S>,
    T: Clone + PartialEq + TryFrom<usize>,
    T: Rem<S, Output = T> + Sum,
    for<'a> T: AddAssign<&'a T>,
{
    let _0 = T::try_from(0).ok().unwrap();
    range
        .into_iter()
        .filter(|k| ds.into_iter().any(|d| k.clone() % d == _0))
        .sum()
}

fn main() {
    println!("{}", solve(1..1000, &vec![3, 5]));
}
