use std::collections::HashSet;
use itertools::Itertools;

fn proper_fractions(n: u64) -> u64 {
    if n == 1 {return 0}

    let fact = factorized(n);
    let mut to_add = 0;
    let mut to_subtract = 0;
    for i in 1..fact.len() + 1 {
        for j in fact.iter().combinations(i) {
            let product = j.iter().fold(1, |res, a| res * *a);
            let tadd = n / product;
            if i % 2 == 0 {
                to_add += tadd;
            }
            else {
                to_subtract += tadd;
            }
        }
    }

    n + to_add - to_subtract
}

fn factorized(n: u64) -> Vec<u64> {
    let mut remainder = n;
    let mut divisor = 2;
    let mut result: HashSet<u64> = HashSet::new();
    
    while remainder % divisor == 0 {
        remainder /= divisor;
        result.insert(divisor);
    }
    divisor += 1;
    
    while remainder > 1 {
        if divisor * divisor > remainder {result.insert(remainder); break;}
        while remainder % divisor == 0 {
            remainder /= divisor;
            result.insert(divisor);
        }
        divisor += 2;
    }

    let result: Vec<u64> = result.into_iter().collect();
    result
}

#[cfg(test)]
mod tests {
    use super::proper_fractions;


    #[test]
    fn sample_tests() {
        assert_eq!(proper_fractions(1), 0, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(2), 1, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(5), 4, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(15), 8, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(25), 20, "\nYour answer (left) is not the expected answer (right).");
    }
}
