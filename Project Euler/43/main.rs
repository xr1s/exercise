use algorithm::number_theory::from_digits;
use itertools::Itertools;

fn is_sub_string_divisibility(perm: &[i64]) -> bool {
    const FACTOR: [i64; 7] = [2, 3, 5, 7, 11, 13, 17];
    perm.windows(3)
        .skip(1)
        .enumerate()
        .all(|(k, sub)| from_digits::<i64>(sub) % FACTOR[k] == 0)
}

fn main() {
    println!(
        "{}",
        (0..=9i64)
            .permutations(10)
            .filter(|perm| is_sub_string_divisibility(perm))
            .map(|ds| from_digits(&ds))
            .sum::<i64>()
    );
}
