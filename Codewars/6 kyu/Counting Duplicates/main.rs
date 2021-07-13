fn count_duplicates(text: &str) -> u32 {
    let mut counter = std::collections::HashMap::new();
    for char in text.chars() {
        counter
            .entry(char.to_ascii_lowercase())
            .and_modify(|v| *v += 1)
            .or_insert(0);
    }
    counter.into_iter().filter(|&(_, val)| val != 0).count() as _
}

fn main() {
    assert_eq!(count_duplicates("abcde"), 0);
    assert_eq!(count_duplicates("aabbcde"), 2);
    assert_eq!(count_duplicates("aabBcde"), 2);
    assert_eq!(count_duplicates("abcdea"), 1);
    assert_eq!(count_duplicates("indivisibility"), 1);
    assert_eq!(count_duplicates("Indivisibilities"), 2);
    assert_eq!(count_duplicates("aA11"), 2);
    assert_eq!(count_duplicates("ABBA"), 2);
}
