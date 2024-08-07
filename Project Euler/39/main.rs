fn solve(maximum: usize) -> usize {
    let mut count = vec![0; maximum + 1];
    for (n, count) in count.iter_mut().enumerate().take(maximum + 1).skip(5) {
        for a in 1..=n / 3 {
            for b in a..=n / 2 {
                let c = n - a - b;
                if c < a || c < b {
                    continue;
                }
                if a + b < c {
                    continue;
                }
                if a * a + b * b != c * c {
                    continue;
                }
                *count += 1;
            }
        }
    }
    let (mut maximum_item, mut maximum) = (0, 0);
    for (n, value) in count.iter().copied().enumerate() {
        if value > maximum {
            maximum_item = n;
            maximum = value;
        }
    }
    maximum_item
}

fn main() {
    println!("{}", solve(1000));
}
