fn solve(n: i64) -> i64 {
    algorithm::number_theory::sieve::primes::<Vec<_>>(n as _)
        .iter()
        .sum()
}

fn main() {
    println!("{}", solve(200_0000));
}
