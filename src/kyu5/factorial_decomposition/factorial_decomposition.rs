use itertools::Itertools;
use std::collections::HashMap;

fn get_primes(n: u32) -> Vec<u32> {
    if n <= 1 {return vec![]};
    if n == 2 {return vec![2]};

    let mut counter = 3;
    let mut primes = vec![2];
    let mut is_prime = true;
    while counter <= n {
        for divisor in primes.iter() {
            if divisor * divisor > counter {break}
            if counter % divisor == 0 {
                is_prime = false;
                break
            }
        }
        if is_prime {primes.push(counter)}
        counter += 2;
        is_prime = true;
    }
    primes
}

pub fn decomp(n: i32) -> String {
    let primes = get_primes(n as u32);
    let mut fact = HashMap::<u32, u32>::new();

    let mut current_value: u32;
    for i in 2..n+1 {
        current_value = i as u32;
        for &j in &primes {
            if j > current_value {break}
            while current_value % j == 0 {
                current_value /= j;
                *fact.entry(j).or_insert(0) += 1;
            }
        }
    }

    fact
        .iter()
        .sorted()
        .map(|(&mantissa, &exponent)| {
            match exponent {
                1 => format!("{mantissa}"),
                _ => format!("{mantissa}^{exponent}"),
            }
        })
        .join(" * ")
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        dotest(5, "2^3 * 3 * 5");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");

    }    
}
