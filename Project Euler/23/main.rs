pub fn p23(n: i64) -> i64 {
    let sigma = algorithm::number_theory::sieve::sigma::<usize>(n as usize + 1);
    let abundant_numbers = (1..n as usize + 1)
        .filter(|&a| sigma[a] - a > a)
        .collect::<Vec<_>>();
    let mut can_sum = std::collections::HashSet::with_capacity(n as usize + 1);
    for &a in &abundant_numbers {
        for &b in &abundant_numbers {
            if a + b < n as usize + 1 {
                can_sum.insert(a + b);
            }
        }
    }
    (1..n as usize)
        .filter(|can| !can_sum.contains(can))
        .sum::<usize>() as _
}

fn main() {
    println!("{}", p23(28123));
}
