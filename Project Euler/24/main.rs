use std::{fmt::Debug, str::FromStr};

// 解题思路是逆康托展开
fn reverse_cantor_expansion<T: Clone>(order: &Vec<T>, mut index: usize) -> Vec<T> {
    let mut order = order.clone();
    let mut perm = Vec::with_capacity(order.len());
    (0..order.len())
        .map(|n| algorithm::number_theory::factorial(&n))
        .rev()
        .for_each(|factorial| {
            let swap = index / factorial;
            let item = order.remove(swap as usize);
            perm.push(item);
            index %= factorial;
        });
    return perm;
}

pub fn solve<T>(order: &Vec<T>, n: usize) -> T
where
    T: Clone + FromStr + ToString,
    <T as FromStr>::Err: Debug,
{
    reverse_cantor_expansion(&order, n - 1)
        .iter()
        .map(T::to_string)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

fn main() {
    println!("{:?}", solve::<i64>(&(0..10).collect(), 100_0000));
}
