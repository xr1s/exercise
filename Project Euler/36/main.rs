struct Palindrome<I, const BASE: usize = 10> {
    underlying: I,
    dup_middle: bool,
    base: I,
    upper_bound: I,
}

impl<I, const B: usize> Palindrome<I, B> {
    fn with_upper(upper_bound: I) -> Self
    where
        I: num::FromPrimitive + num::Zero,
    {
        Self {
            underlying: I::zero(),
            dup_middle: false,
            base: I::from_usize(B).unwrap(),
            upper_bound,
        }
    }
}

impl<I, const B: usize> Iterator for Palindrome<I, B>
where
    I: Clone + num::FromPrimitive + num::One + num::Zero,
    for<'a> &'a I: std::ops::Rem<&'a I, Output = I>,
    for<'a> I: std::ops::MulAssign<&'a I>,
    for<'a> I: std::ops::DivAssign<&'a I>,
    for<'a> I: std::ops::AddAssign<&'a I>,
    for<'a> &'a I: std::cmp::Ord,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.dup_middle ^= true;
            if self.dup_middle {
                self.underlying += &I::one();
            }
            let mut value = self.underlying.clone();
            if self.dup_middle {
                value /= &self.base;
            }
            let mut remain = self.underlying.clone();
            while !remain.is_zero() {
                value *= &self.base;
                let digit = &remain % &self.base;
                value += &digit;
                remain /= &self.base;
            }
            if !self.dup_middle && &value >= &self.upper_bound {
                continue;
            }
            if self.dup_middle && &value >= &self.upper_bound {
                return None;
            } else {
                return Some(value);
            }
        }
    }
}

fn is_bin_palindrome(number: &usize) -> bool {
    if *number == 0 {
        return true;
    }
    let bit_len = number.ilog2() + 1;
    for k in 0..bit_len / 2 {
        let l = number >> (bit_len - 1 - k) & 1;
        let r = number >> k & 1;
        if l != r {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        Palindrome::<usize, 10>::with_upper(1_000_000)
            .filter(is_bin_palindrome)
            .sum::<usize>()
    );
}
