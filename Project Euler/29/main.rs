fn solve(m: i64, n: i64) -> i64 {
    use num::BigInt;
    use std::collections::HashSet;
    let mut pset = HashSet::new();
    for a in 2..m {
        let a = BigInt::from(a);
        for b in 2..n {
            pset.insert(num::pow(a.clone(), b as _));
        }
    }
    return pset.len() as _;
}

fn main() {
    println!("{}", solve(101, 101))
}
