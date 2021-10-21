fn fibonacci_string(n: usize) -> String {
    let mut mem: once_cell::sync::Lazy<Vec<String>> =
        once_cell::sync::Lazy::new(|| vec!["b".into(), "a".into()]);
    if n > mem.len() {
        mem.reserve(n);
    }
    for k in mem.len()..n {
        let len = mem[k - 1].len() + mem[k - 2].len();
        let mut next = String::with_capacity(len);
        next.push_str(&mem[k - 1]);
        next.push_str(&mem[k - 2]);
        mem.push(next);
    }
    return mem[n - 1].clone();
}

fn failure_function(p: impl AsRef<str>) -> Vec<usize> {
    let p = p.as_ref().chars().collect::<Vec<_>>();
    let mut failure = vec![0usize; p.len()];
    let mut s = 0;
    for k in 1..p.len() {
        while s != 0 && p[s] != p[k] {
            s = failure[s - 1];
        }
        if p[s] == p[k] {
            s = s + 1;
        }
        failure[k] = s;
    }
    return failure;
}

fn digits_length(n: usize) -> usize {
    n.to_string().len()
}

fn print_markdown_failure_function_table(s: impl AsRef<str>) {
    let failure = failure_function(s.as_ref());
    print!("\n|  *s* |");
    for (k, c) in s.as_ref().char_indices() {
        print!("{:^1$}|", c, digits_length(failure[k]) + 2);
    }
    print!("\n|:----:|");
    for &k in failure.iter() {
        print!(":{:-^1$}:|", "", digits_length(k));
    }
    print!("\n|*f(s)*|");
    for k in failure.iter() {
        print!(" {} |", k);
    }
    println!();
}

fn main() {
    print_markdown_failure_function_table(fibonacci_string(6));
    print_markdown_failure_function_table(fibonacci_string(7));
}
