// 只想到暴力解法, 但是可以剪枝
//
// 对于 n²+an+b 来说, 因为存在欧拉的解 (1, 41), 我们的解至少达到欧拉解的长度
// 因此 n=0 和 n=1 也至少应当为素数, 代入原式有 b 和 1+a+b 为素数
// 于是枚举素数直接可求得 (a, b), 判断一下 a 和 b 的范围, 算出该解的连续长度, 找到最长的即可
pub fn solve(m: i64, n: i64) -> i64 {
    let primes = algorithm::number_theory::sieve::primes::<i64>(10000);
    let prime_set: std::collections::HashSet<_> = primes.iter().collect();
    let mut max_consecutive = (0, 0, 0);
    for &b in &primes {
        if b.abs() > n {
            break;
        }
        for &p in &primes {
            let a = p - b - 1;
            if a.abs() > m {
                continue;
            }
            let consecutive = (1..)
                .map(|n| n * n + n * a + b)
                .take_while(|p| prime_set.contains(p))
                .collect::<Vec<_>>()
                .len();
            if consecutive > max_consecutive.2 {
                max_consecutive = (a, b, consecutive);
            }
        }
    }
    return max_consecutive.0 * max_consecutive.1;
}

fn main() {
    println!("{}", solve(1001, 1000));
}
