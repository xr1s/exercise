#![feature(format_args_capture)]

fn solve(n: i64) -> i64 {
    let ranges = 1..n;
    let (a, b) = itertools::iproduct!(ranges.clone(), ranges)
        .filter(|&(a, b)| a < b && b < n - a - b)
        .filter(|&(a, b)| 2 * a * b + n * n == 2 * n * (a + b))
        .next()
        .unwrap();
    debug_assert!(a * a + b * b == (n - a - b).pow(2));
    a * b * (n - a - b)
}

fn main() {
    println!("{}", solve(1000));
}
