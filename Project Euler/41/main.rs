fn solve() -> i32 {
    use itertools::Itertools;
    let primes: std::collections::HashSet<i32> = algorithm::number_theory::sieve::primes(7_654_321);
    for p in [7, 6, 5, 4, 3, 2, 1].into_iter().permutations(7) {
        let n: i32 = algorithm::number_theory::from_digits(&p);
        if primes.contains(&n) {
            return n;
        }
    }
    unreachable!(
        r#"There should be answers now.
In case of not finding pandigital prime above, we can continue permutating [1, ..., 6] and so on."#
    );
}

fn main() {
    println!("{}", solve());
}
