fn digit_factorial_sum(n: &i32) -> i32 {
    algorithm::number_theory::digits(n)
        .iter()
        .map(algorithm::number_theory::factorial)
        .sum()
}

fn main() {
    println!(
        "{}",
        (3..2000000)
            .filter(|&n| n == digit_factorial_sum(&n))
            .sum::<i32>()
    );
}
