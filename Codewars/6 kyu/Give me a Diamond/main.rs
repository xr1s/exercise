fn print(n: i32) -> Option<String> {
    if n.is_negative() || n % 2 == 0 {
        return None;
    }
    let s = (0..n)
        .map(|row| {
            let spaces = i32::abs(n / 2 - row) as usize;
            " ".repeat(spaces) + &"*".repeat(n as usize - spaces * 2) + "\n"
        })
        .collect::<Vec<_>>()
        .concat();
    return Some(s);
}

fn main() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
