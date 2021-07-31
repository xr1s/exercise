fn solve(m: i64) -> i64 {
    itertools::iterate((0, 1), |&(p, n)| (n, p + n))
        .map(|(n, ..)| n)
        .take_while(|&n| n < m)
        .filter(|&n| n % 2 == 1)
        .sum()
}

fn main() {
    println!("{}", solve(400_0000));
}
