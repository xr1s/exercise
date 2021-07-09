use itertools::Itertools;

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odd = arr.iter().filter(|&n| n % 2 == 1).sorted();
    arr.iter()
        .map(|n| if n % 2 == 1 { odd.next().unwrap() } else { n })
        .cloned()
        .collect()
}

fn main() {
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
    assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
    assert_eq!(sort_array(&[]), []);
}
