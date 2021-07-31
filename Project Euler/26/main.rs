// 这题模拟除法找循环节也能过, 因为数据范围太小了
// 不过既然是欧拉计划, 那还是要考虑一下数论解的
//
// 纯数论解是
// 对于质因数中只有 2 和 5 的数, 它没有循环节, 必为有限小数
// 对于质因数中含有 2 和 5 的数, 其循环节长度等于因数中除去 2 和 5 之后的数的循环节长度
// 除了质因数中含有 2 和 5 的数外, 1/d 的循环节长度为 10 模 d 的阶
// 所谓的阶就是存在最小的 r, 使得 10 ^ r = 1 (mod d)
// 直观理解就是找到最小的全部由 9 组成的能被 d 整除的数, 其长度即为循环节长度
//
// 根据欧拉定理, 任何数模 d 的阶必为 phi(d) 的因数, 此处 phi 为欧拉函数
// 所以求 phi 后分解因数然后从小到大判断一下就好了.
//
// 参考资料
// https://oeis.org/A051626

pub fn solve(n: i64) -> i64 {
    use algorithm::number_theory::{divisors, modpow};
    algorithm::number_theory::sieve::phi(n as _)
        .iter()
        // 打包为 (d, phi(d))
        .enumerate()
        // d=0 情况是除 0, 略过
        .skip(1)
        // 将所有 d 中 2 和 5 的因数都去除
        .map(|(d, phi)| {
            let mut s = d.clone();
            while s % 2 == 0 {
                s /= 2;
            }
            while s % 5 == 0 {
                s /= 5;
            }
            return (phi, s, d);
        })
        // 过滤只有 2 和 5 的因数的 d, 这种是有限小数无循环节
        .filter(|&(_, d, _)| d != 1)
        // 求 phi(d) 的所有因数
        .map(|(phi, d, origin)| (divisors(*phi), d, origin))
        // 找到因数中满足 10 ^ r = 1 (mod d) 的最小的 r
        .map(|(divisors, d, origin)| {
            (
                divisors
                    .iter()
                    .find(|&&r| modpow(10, r, d as _) == 1)
                    .unwrap()
                    .clone(),
                origin,
            )
        })
        // 按题目要求, 找到 1000 以内最大的 r
        .max()
        .unwrap()
        // 返回该 r 对应的 d
        .1 as _
}

// 这个是后写的, 所以上面没有采用这个函数
/// 计算有理数 1/n 的小数循环节长度, 当有理数为不循环小数或者 n=0 时, 返回 0
pub fn repetend_length(n: usize) -> Vec<i64> {
    use algorithm::number_theory::{divisors, modpow};
    let phi = algorithm::number_theory::sieve::phi(n);
    let mut rep = vec![0; n];
    for (d, phi) in phi.iter().enumerate().skip(1) {
        let mut s = d.clone();
        while s % 2 == 0 {
            s /= 2;
        }
        while s % 5 == 0 {
            s /= 5;
        }
        if s == 1 {
            continue;
        }
        rep[d] = *divisors(*phi)
            .iter()
            .find(|&&p| modpow(10, p, s as _) == 1)
            .unwrap() as _;
    }
    return rep;
}

fn main() {
    println!("{}", solve(1000));
}
