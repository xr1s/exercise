fn disemvowel(s: &str) -> String {
    return s.chars().filter(|&c| !"AEIOUaeiou".contains(c)).collect();
}

fn main() {
    assert_eq!(
        disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}
