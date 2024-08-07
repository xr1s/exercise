pub mod sieve;

use std::ops::{AddAssign, MulAssign};

pub fn digits<N>(n: &N) -> Vec<i32>
where
    N: Clone,
    N: std::ops::DivAssign<i32>,
    N: num::Zero + num::ToPrimitive,
    for<'a> &'a N: std::ops::Rem<i32, Output = N>,
{
    let mut ds = Vec::new();
    let mut n = n.clone();
    while !n.is_zero() {
        let d = &n % 10;
        ds.push(d.to_i32().unwrap());
        n /= 10;
    }
    ds
}

pub fn from_digits<N>(ds: &[N]) -> N
where
    N: Clone + num::One + num::Zero + num::ToPrimitive + num::FromPrimitive,
    N: std::ops::AddAssign<N>,
    N: std::ops::MulAssign<N>,
    for<'a> &'a N: std::ops::Mul<&'a N, Output = N>,
{
    let mut p10 = N::one();
    let mut n = N::zero();
    for d in ds.iter().rev() {
        n += &p10 * d;
        p10 *= N::from_i32(10).unwrap();
    }
    n
}

pub fn factorize<N>(n: &N) -> Vec<(N, i32)>
where
    N: Clone,
    N: std::ops::ShrAssign<i32>,
    for<'a> N: std::ops::AddAssign<&'a N>,
    for<'a> N: std::ops::DivAssign<&'a N>,
    for<'a> &'a N: std::ops::Mul<&'a N, Output = N>,
    N: num::FromPrimitive + num::Integer,
{
    if n.is_zero() || n.is_one() {
        return Vec::new();
    }
    let mut n = n.clone();
    let two = N::from_i32(2).unwrap();
    let mut facts = Vec::new();
    // 单独计算 2 的幂次
    {
        let mut p = 0;
        while n.is_even() {
            n.shr_assign(1);
            p += 1;
        }
        facts.push((two.clone(), p));
    }
    let mut k = N::from_i32(3).unwrap();
    while &k * &k <= n {
        let mut p = 0;
        while n.is_multiple_of(&k) {
            n /= &k;
            p += 1;
        }
        if p != 0 {
            facts.push((k.clone(), p));
        }
        k += &two;
    }
    if !n.is_one() {
        facts.push((n, 1));
    }
    facts
}

pub fn divisors(number: i64) -> Vec<i64> {
    use std::collections::HashMap;
    fn recursive(
        mem: &mut HashMap<usize, Vec<i64>>,
        prime_factors: &Vec<(i64, i32)>,
        index: usize,
    ) -> Vec<i64> {
        if index == prime_factors.len() {
            return vec![1];
        }
        if let Some(result) = mem.get(&index) {
            return result.clone();
        }
        let mut result = Vec::new();
        let (prime, power) = prime_factors[index];
        let mut prime_power = 1;
        for _ in 0..=power {
            let mut s = recursive(mem, prime_factors, index + 1);
            s.iter_mut().for_each(|t| *t *= prime_power);
            result.append(&mut s);
            prime_power *= prime;
        }
        if index != 0 {
            mem.entry(index).or_insert(result.clone());
        }
        result
    }
    let prime_factors = factorize(&number);
    let mut mem = HashMap::with_capacity(prime_factors.len());
    let mut result = recursive(&mut mem, &prime_factors, 0);
    result.sort();
    result
}

/// 求阶乘, 可以传 BigInt
pub fn factorial<N>(n: &N) -> N
where
    N: Clone + PartialEq + num::Zero + num::One,
    for<'a> N: AddAssign<&'a N>,
    for<'a> N: MulAssign<&'a N>,
{
    let one = N::one();
    let mut r = one.clone();
    let mut k = N::zero();
    while &k != n {
        k += &one;
        r *= &k;
    }
    r
}

pub fn combination(m: i64, n: i64) -> u128 {
    let (m, n) = (m as u128, n as u128);
    (1u128..=n).fold(1, |acc, k| acc * (m - n + k) / k)
}

pub fn modpow(mut base: i64, mut exponent: i64, modulo: i64) -> i64 {
    let mut result = 1;
    base %= modulo;
    loop {
        if exponent % 2 == 1 {
            result = result * base % modulo;
        }
        if exponent == 1 {
            break;
        }
        base = base * base % modulo;
        exponent /= 2;
    }
    result
}
