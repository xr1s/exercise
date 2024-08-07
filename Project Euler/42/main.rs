#![allow(incomplete_include)]
#![feature(isqrt)]
#![feature(lazy_cell)]

fn is_triangle_number(t: &i32) -> bool {
    let n = i32::isqrt(t * 2);
    n * n + n == t * 2
}

static WORDS: std::sync::LazyLock<Vec<&str>> = std::sync::LazyLock::new(|| {
    include_str!("words.txt")
        .trim()
        .split(',')
        .map(str::trim)
        .map(|word| word.trim_matches('"'))
        .collect()
});

fn main() {
    println!(
        "{}",
        WORDS
            .iter()
            .map(|word| {
                word.bytes()
                    .map(|byte| byte - b'A' + 1)
                    .map(|byte| byte as i32)
                    .sum()
            })
            .filter(is_triangle_number)
            .count()
    );
}
