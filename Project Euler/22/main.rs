fn solve() -> i64 {
    let names = include_bytes!("p022_names.txt");
    let mut names: Vec<String> = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&names[..])
        .deserialize()
        .next()
        .unwrap()
        .unwrap();
    names.sort();
    names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            name.as_bytes()
                .iter()
                .map(|byte| byte - b'A' + 1)
                .map(|byte| byte as i64)
                .sum::<i64>()
                * (index as i64 + 1)
        })
        .sum()
}

fn main() {
    println!("{}", solve());
}
