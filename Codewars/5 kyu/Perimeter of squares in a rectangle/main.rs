fn perimeter(n: u64) -> u64 {
    itertools::iterate((12, 20), |&(p, n)| (n, p + n))
        .take(n as _)
        .last()
        .unwrap()
        .0
        - 4
}

fn dotest(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

fn main() {
    dotest(5, 80);
    dotest(7, 216);
    dotest(20, 114624);
    dotest(30, 14098308);
}
