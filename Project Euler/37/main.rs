#![feature(isqrt)]
#![feature(lazy_cell)]

fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    if n != 2 && n % 2 == 0 {
        return false;
    }
    for k in (3..=n.isqrt()).step_by(2) {
        if n % k == 0 {
            return false;
        }
    }
    true
}

fn is_not_left_truncate_prime(n: &i32) -> bool {
    let mut n = *n;
    let mut p10 = 10i32.pow(n.ilog10());
    while n != 0 {
        if !is_prime(n) {
            return false;
        }
        n %= p10;
        p10 /= 10;
    }
    true
}

// generate primes from most significant digit
fn gen_from_msd(n: i32) -> Vec<i32> {
    if !is_prime(n) {
        return Vec::new();
    };
    [1, 3, 5, 7, 9]
        .into_iter()
        .map(|d| n * 10 + d)
        .flat_map(gen_from_msd)
        .filter(is_not_left_truncate_prime)
        .chain(if n < 10 { None } else { Some(n) })
        .collect()
}

fn solve() -> i32 {
    [2, 3, 5, 7].into_iter().flat_map(gen_from_msd).sum()
}

fn main() {
    println!("{}", solve());
}
