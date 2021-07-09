fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut vs = arr_a
        .into_iter()
        .filter(|&needle| arr_b.into_iter().any(|&haystack| haystack.contains(needle)))
        .cloned()
        .map(str::to_string)
        .collect::<Vec<_>>();
    vs.sort();
    vs.dedup();
    return vs;
}

fn main() {
    assert_eq!(
        in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        ["live", "strong"]
    );

    assert_eq!(
        in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        ["arp", "live", "strong"]
    );

    assert_eq!(
        in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        [] as [&str; 0]
    );

    assert_eq!(
        in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ),
        ["arp", "live", "strong"]
    );
}
