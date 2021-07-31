use num::pow;

fn solve(n: i64) -> i64 {
    let ranges = pow(10, n as usize - 1)..pow(10, n as usize);
    itertools::iproduct!(ranges.clone(), ranges)
        .map(|(i, j)| i * j)
        .map(|number| number.to_string())
        .filter(|string| string.bytes().eq(string.bytes().rev()))
        .filter_map(|string| string.parse().ok())
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(3));
}
