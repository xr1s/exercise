// 数位和
fn solve(n: i64, p: usize) -> i64 {
    num::pow::pow(num::BigInt::from(n), p)
        .to_string()
        .bytes()
        .map(|byte| byte - b'0')
        .map(|byte| byte as i64)
        .sum()
}

fn main() {
    println!("{}", solve(2, 1000));
}
