use itertools::Itertools;
use num::Integer;

lazy_static::lazy_static! {
    #[rustfmt::skip]
    static ref MORSE_CODE: std::collections::HashMap<String, String> = [
        // alpha
        (".-", "A"), ("-...", "B"), ("-.-.", "C"), ("-..", "D"), (".", "E"),
        ("..-.", "F"), ("--.", "G"), ("....", "H"), ("..", "I"), (".---", "J"),
        ("-.-", "K"), (".-..", "L"), ("--", "M"), ("-.", "N"), ("---", "O"),
        (".--.", "P"), ("--.-", "Q"), (".-.", "R"), ("...", "S"), ("-", "T"),
        ("..-", "U"), ("...-", "V"), (".--", "W"), ("-..-", "X"), ("-.--", "Y"),
        ("Z", "--.."),
        // digit
        ("-----", "0"), (".----", "1"), ("..---", "2"), ("...--", "3"), ("....-", "4"),
        (".....", "5"), ("-....", "6"), ("--...", "7"), ("---..", "8"), ("----.", "9"),
        // punct
        (".-.-.-", "."), ("--..--", ","), ("..--..", "?"), (".----.", "\'"), ("-.-.--", "!"),
        ("-..-.", "/"), ("-.--.", "("), ("-.--.-", ")"), (".-...", "&"), ("---...", ":"),
        ("-.-.-.", ";"), ("-...-", "="), (".-.-.", "+"), ("-....-", "-"), ("..--.-", "_"),
        (".-..-.", "\""), ("...-..-", "$"), (".--.-.", "@"), ("..-.-", "¿"), ("--...-", "¡"),
    ]
    .into_iter()
    .map(|(key, val)| (key.into(), val.into()))
    .collect();
}

pub fn decode_bits(encoded: &str) -> String {
    lazy_static::lazy_static! {
        static ref GROUP: regex::Regex = regex::Regex::new("0+|1+").unwrap();
    };
    let encoded = encoded.trim_matches('0');
    let step = GROUP
        .find_iter(&encoded)
        .map(|_match| _match.as_str().len())
        .fold(0, |div, next| div.gcd(&next));
    return encoded
        .chars()
        .step_by(step)
        .collect::<String>()
        .replace("111", "-")
        .replace("1", ".")
        .replace("0000000", " ")
        .replace("000", "/")
        .replace("0", "");
}

pub fn decode_morse(encoded: &str) -> String {
    return encoded
        .split(' ')
        .map(|w| w.split('/').filter_map(|m| MORSE_CODE.get(m)).join(""))
        .join(" ");
}

fn main() {
    println!("{}", decode_morse(&decode_bits("01100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")));
}
