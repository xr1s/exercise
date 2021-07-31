fn divisors_count(number: i64) -> i64 {
    algorithm::number_theory::factorize(number)
        .iter()
        .map(|&(_, p)| p as i64 + 1)
        .product()
}

fn solve(n: i64) -> i64 {
    (0..)
        .map(|number| number * (number + 1) / 2)
        .map(|triangle| (triangle, divisors_count(triangle)))
        .find(|&(_, divisor)| divisor >= n)
        .unwrap()
        .0
}

fn main() {
    println!("{}", solve(500));
}
