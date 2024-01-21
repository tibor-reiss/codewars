struct Prime {
    curr: u32,
    next: u32,
    primes: Vec<u32>,
}

impl Prime {
    fn new() -> Self {
        Prime{curr: 2, next: 2, primes: Vec::new()}
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next;
        if self.curr == 2 {
            self.next = 3;
            return Some(self.curr);
        }
        
        let mut increased = false;
        self.next += 2;
        loop {
            for divisor in self.primes.iter() {
                if divisor * divisor > self.next {
                    break
                }
                if self.next % divisor == 0 {
                    self.next += 2;
                    increased = true;
                    break
                }
            }
            if !increased {
                break
            }
            increased = false;
        }
        self.primes.push(self.curr);
        Some(self.curr)
    }
}

pub fn stream() -> impl Iterator<Item = u32> {
    Prime::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_segment(start: u32, numbers: [u32; 10]){
        let mut prime_iterator = stream();
        for _ in 0..start{
            prime_iterator.next();
        }
        for i in numbers{
            assert_eq!(Some(i), prime_iterator.next(),
            "\nYour result (left) did not match the expected output (right)");
        }
    }
    
    #[test]
    fn tests() {
        println!("testing segment from 0");
        test_segment(0, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        
        println!("testing segment from 10");
        test_segment(10, [31, 37, 41, 43, 47, 53, 59, 61, 67, 71]);
        
        println!("testing segment from 100");
        test_segment(100, [547, 557, 563, 569, 571, 577, 587, 593, 599, 601]);
        
        println!("testing segment from 1,000");
        test_segment(1_000, [7927, 7933, 7937, 7949, 7951, 7963, 7993, 8009, 8011, 8017]);
    }
}

