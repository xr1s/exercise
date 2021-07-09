fn persistence(num: u64) -> u64 {
    let p = num
        .to_string()
        .bytes()
        .map(|byte| byte - b'0')
        .map(|byte| byte as u64)
        .product();
    return if p == num { 0 } else { persistence(p) + 1 };
}

fn main() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
}
