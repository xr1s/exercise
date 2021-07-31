pub mod sieve;

use std::ops::{AddAssign, MulAssign};

use num::{One, Zero};

pub fn factorize(mut number: i64) -> Vec<(i64, i32)> {
    let mut k = 2;
    let mut facts = Vec::new();
    while k * k <= number {
        let mut power = 0;
        while number % k == 0 {
            number /= k;
            power += 1;
        }
        if power != 0 {
            facts.push((k, power));
        }
        k += 1;
    }
    if number > 1 {
        facts.push((number, 1));
    }
    return facts;
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
        return result;
    }
    let prime_factors = factorize(number);
    let mut mem = HashMap::with_capacity(prime_factors.len());
    let mut result = recursive(&mut mem, &prime_factors, 0);
    result.sort();
    return result;
}

/// 求阶乘, 可以传 BigInt
pub fn factorial<T>(n: &T) -> T
where
    T: PartialEq + AddAssign + Zero + One,
    for<'a> T: MulAssign<&'a T>,
{
    let mut r = T::one();
    let mut k = T::zero();
    while &k != n {
        k += T::one();
        r *= &k;
    }
    return r;
}

pub fn combination(m: i64, n: i64) -> u128 {
    let (m, n) = (m as u128, n as u128);
    (1u128..=n as u128).fold(1, |acc, k| acc * (m - n + k) / k)
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
    return result;
}
