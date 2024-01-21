use std::f32::consts::PI;

fn example_func(f: f64) -> f64 {
    3.0 / 2.0 * f.sin().powf(3.0)
}

pub fn simpson(n: i32) -> f64 {
    let a: f64 = 0.0;
    let b: f64 = PI as f64;
    let h: f64 = (b - a) / n as f64;
    let mut result: f64 = example_func(a) + example_func(b);

    for i in 1..(n / 2 + 1) {
        result += 4.0 * example_func(a + (2.0 * i as f64 - 1.0) * h);
    }
    for i in 1..(n / 2) {
        result += 2.0 * example_func(a + 2.0 * i as f64 * h);
    }

    (b - a) / (3.0 * n as f64) * result
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;
    
    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-10;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }

    fn dotest(n: i32, exp: f64) -> () {
        assert_float_equals(simpson(n), exp);
    }

    #[test]
    fn basic_tests_dist() {
        dotest(290, 1.9999999986);
        dotest(72, 1.9999996367);
        dotest(252, 1.9999999975);
    }
}

