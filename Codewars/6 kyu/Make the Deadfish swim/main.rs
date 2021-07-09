fn parse(code: &str) -> Vec<i32> {
    let mut array = Vec::new();
    let mut current = 0;
    for command in code.chars() {
        match command {
            'i' => current += 1,
            'd' => current -= 1,
            's' => current *= current,
            'o' => array.push(current),
            _ => (),
        }
    }
    return array;
}

fn main() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
