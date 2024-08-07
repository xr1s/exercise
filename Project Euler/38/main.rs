fn calculate_boundary(k: i32) -> (i32, i32) {
    let len = 9 / k;
    let upper = 10i32.pow(len as _);
    let mut lower = 0;
    let mut p10 = 1;
    for k in (1..=len).rev() {
        lower += k * p10;
        p10 *= 10;
    }
    (lower, upper)
}

fn is_pandigital(n: i32) -> bool {
    let mut n = n as usize;
    let mut set = bit_set::BitSet::with_capacity(9);
    while n != 0 {
        let d = n % 10;
        if d == 0 {
            return false;
        }
        set.insert(d);
        n /= 10;
    }
    set.len() == 9
}

fn concatenated_product(n: i32, k: i32) -> i32 {
    let mut concat = 0;
    let mut concat_len = 0u32;
    for i in 1..=k {
        let prod = n * i;
        let len = prod.ilog10() + 1;
        concat_len += len;
        if concat_len > 9 {
            return 0;
        }
        concat *= i32::pow(10, len);
        concat += prod;
    }
    if concat_len != 9 {
        return 0;
    }
    if !is_pandigital(concat) {
        return 0;
    }
    concat
}

fn solve() -> i32 {
    let mut maximum = 0;
    for k in 2..10 {
        let (lower, upper) = calculate_boundary(k);
        for n in lower..upper {
            maximum = concatenated_product(n, k).max(maximum);
        }
    }
    maximum
}

fn main() {
    println!("{}", solve());
}
