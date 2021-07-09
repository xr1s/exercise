fn pos_average(s: &str) -> f64 {
    use itertools::{iproduct, zip};
    let vs = s.split(", ").collect::<Vec<_>>();
    let same = iproduct!(0..vs.len(), 0..vs.len())
        .filter(|&(i, j)| i < j)
        .map(|(i, j)| (vs[i].chars(), vs[j].chars()))
        .map(|(s1, s2)| zip(s1, s2).filter(|&(c1, c2)| c1 == c2).count())
        .sum::<usize>() as f64;
    let chars = vs.len() * (vs.len() - 1) * vs[0].len() / 2;
    return (same * 1e12 / chars as f64).round() / 1e10;
}

fn main() {
    use float_eq::float_eq;
    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res =
            float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(
            res,
            "Expected value must be near: {:e} but was:{:e}",
            expected, actual
        );
    }
    #[rustfmt::skip]
    assert_float_equals(pos_average("466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096"), 26.6666666667);
    #[rustfmt::skip]
    assert_float_equals(pos_average("444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490"), 29.2592592593);
}
