fn solve(n: i64) -> i64 {
    (1..=n).fold(1, num::integer::lcm)
}

fn main() {
    println!("{}", solve(20));
}
