// 该算法论文地址
// https://arxiv.org/html/2312.15338v1
fn frazer_jarvis_sqrt<I>(n: &I, precision: u32) -> num::BigUint
where
    I: Clone + Into<num::BigUint>,
{
    use num::One;
    let mut a: num::BigUint = n.clone().into();
    let mut b = num::BigUint::one();
    // 输入参数 n 的长度
    let nlen = a.to_radix_be(10).len() - 1;
    // 计算结果的整数部分长度，用于计算最后分离小数点前后数位
    let ipart = nlen as u32 / 2 + 1;
    let precision = num::BigUint::from(10u32).pow(precision + ipart);
    // 避免 a 极大的情况，进入下面的迭代会需要减数亿次 b
    // b*k + [0+2(k-1)]*k/2 <= a, find the maximum k
    // b*k + k^2 - k <= a
    // k² + (b-1)k - a <= 0
    // k <= [1-b + sqrt(b²-2b+1+4a)]/2
    // if a >= b {
    //     let delta: num::BigUint = &b * &b + (&a << 2) + 1u32 - (&b << 1);
    //     // 这里调用了 num 库的 sqrt，会不会有偷懒嫌疑（好像是牛顿迭代）
    //     // 不过无所谓，就算用二分都比减数亿次要优
    //     let k = (delta.sqrt() + 1u32 - &b) >> 1;
    //     a -= &k * (&k + &b - 1u32);
    //     b += k << 1;
    // }
    while b < precision {
        if a >= b {
            a -= &b;
            b += 2u32;
        } else {
            a *= 100u32;
            b *= 10u32;
            b -= 9u32;
        }
    }
    b / 20u32
}

fn main() {
    let squares: std::collections::HashSet<u32> = (0..10).map(|n| n * n).collect();
    let sum: i32 = (0..100u32)
        .filter(|n| !squares.contains(n))
        .map(|n| frazer_jarvis_sqrt(&n, 100))
        .map(|n| {
            let sum = &n.to_radix_be(10);
            sum.iter().map(|&d| d as i32).take(100).sum::<i32>()
        })
        .sum();
    println!("{}", sum);
}
