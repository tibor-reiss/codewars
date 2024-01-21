use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

pub fn sq_cub_rev_prime(n: u32) -> u32 {
    let alen = ARRAY.lock().unwrap().len();
    if n <= alen as u32 {return ARRAY.lock().unwrap()[n as usize - 1]};

    let mut counter = 0;
    let mut nr: u64 = 0;
    if alen > 0 {
        counter = alen as u32;
        nr = ARRAY.lock().unwrap()[alen - 1] as u64;
    }

    while counter < n {
        nr += 1;
        let rev_square = (nr * nr).to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
        let rev_cube = (nr * nr * nr).to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
        if is_prime(rev_square) && is_prime(rev_cube) {
            counter += 1;
            ARRAY.lock().unwrap().push(nr as u32);
        }
    }
    nr as u32
}

fn is_prime(n: u64) -> bool {
    if n == 0 || n == 1 {return false};
    if n == 2 {return true};
    if n % 2 == 0 {return false};

    let mut divisor = 3;
    while divisor * divisor <= n {
        if n % divisor == 0 {return false};
        divisor += 2
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::sq_cub_rev_prime;
        
    fn dotest(n: u32, expected: u32) {
        let actual = sq_cub_rev_prime(n);
        assert!(actual == expected, "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(1, 89);
        dotest(2, 271);
        dotest(3, 325);
        dotest(4, 328);
        dotest(148, 31807);
        dotest(168, 32759);
    }
}
