use std::collections::BTreeSet;

fn is_pandigits(m: i32, n: i32, p: i32) -> bool {
    use algorithm::number_theory::digits;
    let digits = digits(&m)
        .into_iter()
        .chain(digits(&n))
        .chain(digits(&p))
        .collect::<Vec<_>>();
    let bitset = digits
        .iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .fold(0, |acc, next| acc | 1 << next);
    bitset == 0b1111111110 && digits.len() == 9
}

fn find_pandigits<T, U>(m0: T, m1: U) -> BTreeSet<(i32, i32, i32)>
where
    T: IntoIterator<Item = i32>,
    <T as IntoIterator>::IntoIter: Clone,
    U: IntoIterator<Item = i32>,
    <U as IntoIterator>::IntoIter: Clone,
{
    let mut set = BTreeSet::new();
    for (m, n) in itertools::iproduct!(m0, m1) {
        let p = m * n;
        if is_pandigits(m, n, p) {
            set.insert((m, n, p));
        }
    }
    set
}

fn main() {
    let pandigits0 = find_pandigits(1..10, 1000..10000);
    let pandigits1 = find_pandigits(10..100, 100..1000);
    let pandigits = pandigits0.union(&pandigits1).collect::<BTreeSet<_>>();
    for (m, n, p) in &pandigits {
        eprintln!("{m} * {n} = {p}");
    }
    println!(
        "{}",
        pandigits
            .into_iter()
            .map(|(_, _, p)| p)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .sum::<i32>()
    );
}
