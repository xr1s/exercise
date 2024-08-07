use std::ops::{Add, Div, Mul, Rem, Sub};

use bit_vec::BitVec;
use num::{FromPrimitive, ToPrimitive};

fn size_hint(n: usize) -> usize {
    let logarithm = (n as f64).ln().max(1.);
    let size_hint = n as f64 / logarithm * 1.25;
    size_hint as _
}

/// 欧拉筛法求素数
pub fn primes<Container>(n: usize) -> Container
where
    Container::Item: Clone + PartialOrd + FromPrimitive + ToPrimitive + num::Zero,
    for<'a> &'a Container::Item: Mul<&'a Container::Item, Output = Container::Item>,
    for<'a> &'a Container::Item: Rem<&'a Container::Item, Output = Container::Item>,
    Container: IntoIterator + FromIterator<Container::Item>,
{
    let ni = Container::Item::from_usize(n).unwrap();

    let mut primes = Vec::<Container::Item>::with_capacity(size_hint(n));
    let mut composites = BitVec::from_elem(n, false);
    for k in 2..n {
        let ki = Container::Item::from_usize(k).unwrap();
        if !composites[k] {
            primes.push(ki.clone());
        }
        for p in &primes {
            if &ki * p >= ni {
                break;
            }
            composites.set(k * p.to_usize().unwrap(), true);
            let rem = &ki % p;
            if <Container::Item as num::Zero>::is_zero(&rem) {
                break;
            }
        }
    }
    Container::from_iter(primes)
}

/// 除数函数
/// 对于输入 n, 返回 [1, n) 每个整数的所有因数计数
/// 参考资料 https://oeis.org/A000005
pub fn tau<Int>(n: usize) -> Vec<Int>
where
    Int: Clone + FromPrimitive,
    for<'a> Int: Add<&'a Int, Output = Int>,
    for<'a> Int: Mul<&'a Int, Output = Int>,
    for<'a> Int: Div<&'a Int, Output = Int>,
{
    let one = Int::from_usize(1).unwrap();
    let mut primes = Vec::with_capacity(size_hint(n));
    let mut composites = BitVec::from_elem(n, false);
    let mut tau = vec![Int::from_usize(0).unwrap(); n];
    // aux[n] 储存 n 最小素因子的幂次
    let mut aux = vec![Int::from_usize(0).unwrap(); n];
    if n != 0 {
        tau[1] = Int::from_usize(1).unwrap();
    }
    for k in 2..n {
        if !composites[k] {
            aux[k] = Int::from_usize(2).unwrap();
            tau[k] = Int::from_usize(2).unwrap();
            primes.push(k);
        }
        for &p in &primes {
            let c = k * p;
            if c >= n {
                break;
            }
            composites.set(c, true);
            if k % p == 0 {
                aux[c] = aux[k].clone() + &one;
                tau[c] = tau[k].clone() / &aux[k] * &aux[c];
                break;
            }
            aux[c] = Int::from_usize(2).unwrap();
            tau[c] = tau[p].clone() * &tau[k];
        }
    }
    tau
}

/// 除数函数
/// 对于输入 n, 返回从 1 到 n 每个数的所有因数和
/// 参考资料 https://en.wikipedia.org/wiki/Divisor_function
pub fn sigma<Int>(n: usize) -> Vec<Int>
where
    Int: Clone + FromPrimitive,
    for<'a> Int: Add<&'a Int, Output = Int>,
    for<'a> Int: Mul<&'a Int, Output = Int>,
    for<'a> Int: Div<&'a Int, Output = Int>,
{
    let one = Int::from_usize(1).unwrap();
    let mut primes = Vec::with_capacity(size_hint(n));
    let mut composites = BitVec::from_elem(n, false);
    let mut sigma = vec![Int::from_usize(0).unwrap(); n];
    // auxil[k] 储存 k 最小素因数所有幂次的和
    let mut auxil = vec![Int::from_usize(0).unwrap(); n];
    if n != 0 {
        sigma[1] = Int::from_usize(1).unwrap();
    }
    for k in 2..n {
        if !composites[k] {
            primes.push(k);
            auxil[k] = Int::from_usize(k).unwrap() + &one;
            sigma[k] = Int::from_usize(k).unwrap() + &one;
        }
        for &p in &primes {
            let c = k * p;
            if c >= n {
                break;
            }
            composites.set(c, true);
            if k % p == 0 {
                auxil[c] = Int::from_usize(p).unwrap() * &auxil[k] + &one;
                sigma[c] = sigma[k].clone() / &auxil[k] * &auxil[c];
                break;
            }
            auxil[c] = Int::from_usize(p).unwrap() + &one;
            sigma[c] = sigma[p].clone() * &sigma[k];
        }
    }
    sigma
}

/// 欧拉函数
/// 对于输入 n, 返回从 1 到 n 每个数比本身小的和本身互质的数个数
/// 参考资料 https://en.wikipedia.org/wiki/Euler%27s_totient_function
pub fn phi<Int>(n: usize) -> Vec<Int>
where
    Int: Clone + FromPrimitive,
    for<'a> Int: Sub<&'a Int, Output = Int>,
    for<'a> Int: Mul<&'a Int, Output = Int>,
    for<'a> Int: Div<&'a Int, Output = Int>,
{
    let one = Int::from_usize(1).unwrap();
    let mut primes = Vec::with_capacity(size_hint(n));
    let mut composites = BitVec::from_elem(n, false);
    let mut phi = vec![Int::from_usize(0).unwrap(); n];
    if n > 1 {
        phi[1] = Int::from_usize(1).unwrap();
    }
    for k in 2..n {
        if !composites[k] {
            primes.push(k);
            phi[k] = Int::from_usize(k).unwrap() - &one;
        }
        for &p in &primes {
            let c = p * k;
            if c >= n {
                break;
            }
            composites.set(c, true);
            if k % p == 0 {
                phi[c] = Int::from_usize(p).unwrap() * &phi[k];
                break;
            }
            phi[c] = phi[p].clone() * &phi[k];
        }
    }
    phi
}
