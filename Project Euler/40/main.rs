// OEIS 的 A33307 数列
// 需要注意的是和题目不一样，下标从 0 开始
// 也就是小数点后第 1 位应该传入 n=0
fn oeis033307(mut n: i64) -> i64 {
    // 当前枚举到几位数（数字长度）
    let mut d = 1;
    // 10 的幂次，对应几位数的最小值
    let mut p10 = 1;
    loop {
        // 所有 d 位数组成的数位总长度
        let digits = 9 * p10 * d;
        if digits > n {
            break;
        }
        n -= digits;
        p10 *= 10; // 注意此处，复杂度是 log 级别的
        d += 1;
    }
    // number 是所求第小数点后第 n 位所在的完整数字
    let number = p10 + n / d;
    // digit 是所求的第 n 位在 number 中的位置
    let digit = d - n % d - 1;
    number / i64::pow(10, digit as _) % 10
}

fn main() {
    println!(
        "{}",
        [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]
            .into_iter()
            .map(|d| d - 1)
            .map(oeis033307)
            .product::<i64>()
    );
}
