// 典型背包问题, 计算方案数
// state[k] 表示凑出 k 元来有多少方案

fn solve(n: usize) -> i64 {
    let mut state = vec![0; n + 1];
    state[0] = 1;
    for i in [1, 2, 5, 10, 20, 50, 100, 200] {
        for j in i..=n {
            state[j] += state[j - i];
        }
    }
    return state[n];
}

fn main() {
    println!("{}", solve(200));
}
