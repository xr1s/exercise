fn is_curious_fraction(a: i32, b: i32, c: i32, d: i32) -> bool {
    let (n, m) = (a * 10 + b, c * 10 + d);
    if b == 0 && d == 0 || n >= m {
        return false;
    }
    // 从这里开始 ac bd 不同时相等
    // n / m == b / d <=> n * d == m * b
    if a == c && n * d == m * b {
        eprintln!("{a}{b} / {c}{d} = {b} / {d}");
        return true;
    }
    // n / m == b / c <=> n * c == m * b
    if a == d && n * c == m * b {
        eprintln!("{a}{b} / {c}{d} = {b} / {c}");
        return true;
    }
    // n / m == a / d <=> n * d == m * a
    if b == c && n * d == m * a {
        eprintln!("{a}{b} / {c}{d} = {a} / {d}");
        return true;
    }
    // n / m == a / c <=> n * c == m * a
    if b == d && n * c == m * a {
        eprintln!("{a}{b} / {c}{d} = {a} / {c}");
        return true;
    }
    false
}

fn solve() -> i32 {
    let (mut numerator, mut denominator) = (1, 1);
    for (a, b, c, d) in itertools::iproduct!(1..10, 0..10, 1..10, 0..10) {
        if is_curious_fraction(a, b, c, d) {
            numerator *= a * 10 + b;
            denominator *= c * 10 + d;
        }
    }
    let gcd = num::integer::gcd(numerator, denominator);
    denominator / gcd
}
fn main() {
    println!("{}", solve());
}
