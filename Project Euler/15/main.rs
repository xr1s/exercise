// 组合数例题
// 总共固定要走 40 步, 要在 40 步里选 20 步往下走
fn solve(m: i64, n: i64) -> i64 {
    algorithm::number_theory::combination(m + n, m) as _
}

fn main() {
    println!("{}", solve(20, 20))
}
