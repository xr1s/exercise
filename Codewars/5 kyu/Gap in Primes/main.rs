fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    fn is_prime(p: u64) -> bool {
        if p == 1 {
            return false;
        }
        if p % 2 == 0 || p % 3 == 0 || p % 5 == 0 {
            return p == 2 || p == 3 || p == 5;
        }
        for k in (5..).step_by(6) {
            if k * k > p {
                return true;
            }
            if p % k == 0 || p % (k + 2) == 0 {
                return false;
            }
        }
        return true;
    }
    let mut p = None;
    for q in m..=n {
        if is_prime(q) {
            if let Some(p) = p {
                if p + g as u64 == q {
                    return Some((p, q));
                }
            }
            p = Some(q);
        }
    }
    return None;
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

fn main() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
}
