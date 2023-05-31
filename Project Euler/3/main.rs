fn solve(n: i64) -> i64 {
    algorithm::number_theory::factorize(&n).last().unwrap().0
}

fn main() {
    println!("{}", solve(600851475143));
}
