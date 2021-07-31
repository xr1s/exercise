use std::ops::RangeBounds;

pub fn solve<T, R: RangeBounds<T> + Iterator<Item = i64>>(r: R) -> usize {
    r.map(|number| {
        english_numbers::convert(
            number,
            english_numbers::Formatting {
                title_case: false,
                spaces: false,
                conjunctions: true,
                commas: false,
                dashes: false,
            },
        )
    })
    .map(|s| s.len())
    .sum()
}

fn main() {
    println!("{}", solve(1..=1000));
}
