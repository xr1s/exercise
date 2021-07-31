// 9⁵ * 6 有 6 位可以表示 6 位数
// 9⁵ * 7 只有 6 位表示不了 7 位数
// 因此更高位也追不上 (9⁵n 和 10ⁿ 增长阶不一样)
// 所以只要考虑最高 9⁵ * 6 以内的数字即可
fn solve(count: usize) -> i64 {
    use num::pow;
    (2..pow(9, count) * 6)
        .map(|n| {
            (
                n.to_string()
                    .bytes()
                    .map(|byte| byte - b'0')
                    .collect::<Vec<_>>(),
                n,
            )
        })
        .filter(|(ds, n)| *n == ds.iter().map(|&d| pow(d as i64, count)).sum())
        .map(|(.., n)| n)
        .sum()
}

fn main() {
    println!("{}", solve(5));
}
