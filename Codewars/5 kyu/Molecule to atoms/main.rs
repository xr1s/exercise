use std::ops::AddAssign;

use thiserror::Error;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Not a valid molecule")]
    Invalid,
    #[error("Mismatched parenthesis")]
    Parenthesis,
}

#[derive(Debug)]
enum ParseState {
    Start,
    Number(usize),
    Lower(String, usize),
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    use ParseState::*;
    let mut multi = Vec::new();
    let mut parenthesis = Vec::new();
    let mut state = Start;
    let mut rs = std::collections::HashMap::<String, usize>::new();
    for c in s.chars().rev() {
        if c.is_ascii_digit() {
            match state {
                Start => state = Number(c as usize - '0' as usize),
                Number(n) => state = Number(10 * (c as usize - '0' as usize) + n),
                Lower(_, _) => return Err(ParseError::Invalid),
            }
        }
        if c.is_ascii_lowercase() {
            match state {
                Start => state = Lower(c.to_string(), 1),
                Number(n) => state = Lower(c.to_string(), n),
                Lower(m, n) => state = Lower(format!("{}{}", m, c), n),
            }
        }
        if c.is_ascii_uppercase() {
            let multi = multi.iter().product();
            match state {
                Start => {
                    rs.entry(c.to_string())
                        .and_modify(|v| v.add_assign(multi))
                        .or_insert(multi);
                }
                Number(n) => {
                    rs.entry(c.to_string())
                        .and_modify(|v| v.add_assign(n * multi))
                        .or_insert(n * multi);
                }
                Lower(m, n) => {
                    let m = format!("{}{}", c, m.chars().rev().collect::<String>());
                    rs.entry(m)
                        .and_modify(|v| v.add_assign(multi))
                        .or_insert(n * multi);
                }
            }
            state = Start;
        }
        if ")]}".contains(c) {
            match state {
                Start => multi.push(1),
                Number(n) => multi.push(n),
                Lower(_, _) => return Err(ParseError::Invalid),
            }
            parenthesis.push(c);
            state = Start;
        }
        if "([{".contains(c) {
            match (c, parenthesis.pop()) {
                ('(', Some(')')) => (),
                ('[', Some(']')) => (),
                ('{', Some('}')) => (),
                _ => return Err(ParseError::Parenthesis),
            };
            match state {
                Number(_) | Lower(_, _) => return Err(ParseError::Invalid),
                _ => (),
            }
            multi.pop();
            state = Start;
        }
    }
    if parenthesis.len() != 0 {
        return Err(ParseError::Parenthesis);
    }
    match state {
        Start => return Ok(rs.into_iter().collect()),
        Number(_) | Lower(_, _) => return Err(ParseError::Invalid),
    };
}

fn main() {
    fn assert_fail(formula: &str, msg: &str) {
        let result = parse_molecule(formula);
        assert!(
            result.is_err(),
            "expected {} {:?} to fail, got {:?}",
            msg,
            formula,
            result.unwrap()
        );
    }

    fn assert_parse(formula: &str, expected: &[(&str, usize)]) {
        let mut expected = expected
            .into_iter()
            .map(|&(name, usize)| (name.to_owned(), usize))
            .collect::<Molecule>();
        let result = parse_molecule(formula);
        assert!(
            result.is_ok(),
            "expected {:?} to pass, got {:?}",
            formula,
            result
        );
        let mut actual = result.unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
    assert_parse("H", &[("H", 1)]);
    assert_parse("O2", &[("O", 2)]);
    assert_parse("H2O", &[("H", 2), ("O", 1)]);
    assert_parse("Mg(OH)2", &[("Mg", 1), ("O", 2), ("H", 2)]);
    assert_parse("K4[ON(SO3)2]2", &[("K", 4), ("O", 14), ("N", 2), ("S", 4)]);

    assert_fail("pie", "Not a valid molecule");
    assert_fail("Mg(OH", "Mismatched parenthesis");
    assert_fail("Mg(OH}2", "Mismatched parenthesis");
}
