use num::BigInt;

fn solve(n: usize) -> usize {
    let mut prev = BigInt::from(0);
    let mut curr = BigInt::from(1);
    let mut index = 0;
    while prev.to_string().len() < n {
        let next = prev + &curr;
        prev = curr;
        curr = next;
        index += 1;
    }
    return index;
}

fn main() {
    println!("{}", solve(1000));
}
