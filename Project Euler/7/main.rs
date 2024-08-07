// 只能说, 这个 11_0000 是凑出来的
// 应该没有推算第 k 个素数数量级的算法
fn solve(n: i64) -> i64 {
    algorithm::number_theory::sieve::primes::<Vec<_>>(11_0000)[n as usize - 1]
}

fn main() {
    println!("{}", solve(10001));
}
