// 首先这种搜索肯定是要记忆化的
// 然后从 StackOverflow 上看来的一个优化是只记忆化奇数
// 这个很好理解, 因为偶数最长不会超过 19 次, 但是占了一半的记忆化空间
// 优化一半空间可以大幅度提高散列表的命中率
// 这个优化将我的耗时从 696.58ms 优化到了 383.58ms
//
// 参考资料:
// - https://stackoverflow.com/questions/38885614/longest-collatz-or-hailstone-sequence-optimization-python-2-7
// - https://oeis.org/A006877
use std::collections::HashMap;

fn recursive(mem: &mut HashMap<i64, i32>, n: i64) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }
    if let Some(next) = mem.get(&n) {
        return next.clone();
    }
    match n % 2 {
        0 => recursive(mem, n / 2),
        1 => {
            let remains = recursive(mem, n * 3 + 1);
            mem.entry(n).or_insert(remains + 1).clone()
        }
        _ => unreachable!(),
    }
}

pub fn solve(n: i64) -> i64 {
    let mut mem = HashMap::new();
    (0..=n)
        .map(|n| (recursive(&mut mem, n), n))
        .max()
        .unwrap()
        .1
}

fn main() {
    println!("{}", solve(100_0000));
}
