fn parse(code: &str) -> Vec<i32> {
    let mut array = Vec::new();
    let mut value = 0;
    for command in code.chars() {
        match command {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value *= value,
            'o' => array.push(value),
            _ => (),
        }
    }
    return array;
}

fn main() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
