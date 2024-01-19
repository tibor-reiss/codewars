pub fn going(n: i32) -> f64 {
    let mut a: f64 = 1.0;
    ((1.0 + (0..n-1).map(|i: i32| {
        a /= (n-i) as f64;
        a
    }).sum::<f64>()) * 1.0e6).trunc() / 1.0e6
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;
    
    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }

    fn dotest(n: i32, exp: f64) -> () {
        assert_float_equals(going(n), exp);
    }

    #[test]
    fn basics_going() {
        dotest(5, 1.275);
        dotest(6, 1.2125);
        dotest(7, 1.173214);
        dotest(8, 1.146651);
    }
}
