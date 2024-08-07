#![feature(lazy_cell)]

use std::collections::HashSet;

static PRIMES: std::sync::OnceLock<HashSet<i32>> = std::sync::OnceLock::new();

fn rotate(prime: &i32) -> Vec<i32> {
    let len = prime.ilog10() + 1;
    let mut rotations = Vec::with_capacity(len as _);
    rotations.push(*prime);
    let mut p = 10;
    let mut q = 10i32.pow(len - 1);
    for _ in 1..len {
        rotations.push(prime % p * q + prime / p);
        p *= 10;
        q /= 10;
    }
    rotations
}

fn is_circular_prime(rotations: &[i32]) -> bool {
    if rotations.is_empty() {
        return false;
    }
    let prime = rotations[0];
    let mut digits = prime / 10;
    while digits != 0 {
        let d = digits % 10;
        if d % 2 == 0 || d == 5 {
            return false;
        }
        digits /= 10;
    }
    let primes = PRIMES.get().unwrap();
    rotations.iter().all(|rotation| primes.contains(rotation))
}

fn solve(upper: usize) -> usize {
    let primes = PRIMES.get_or_init(|| algorithm::number_theory::sieve::primes(upper));
    let mut circular = HashSet::with_capacity(primes.len());
    for prime in primes.iter() {
        if circular.contains(prime) {
            continue;
        }
        let rotations = rotate(prime);
        if is_circular_prime(&rotations) {
            circular.extend(rotations);
        }
    }
    circular.len()
}

fn main() {
    println!("{:?}", solve(1_000_000));
}
