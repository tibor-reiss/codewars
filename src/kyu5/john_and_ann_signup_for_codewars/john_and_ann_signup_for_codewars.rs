#[derive(Clone)]
struct AnnJohn {
    n: i32,
    ann: Vec<i32>,
    john: Vec<i32>,
}

impl AnnJohn {
    fn new() -> Self {
        AnnJohn{n: 0, ann: vec![], john: vec![]}
    }
}

impl Iterator for AnnJohn {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 || self.n == 1 {
            self.ann.push(1);
            self.john.push(0);
            self.n += 1;
            return Some((1, 0))
        };

        let ann = self.n - self.john[self.ann[self.n as usize - 1] as usize];
        let john = self.n - self.ann[self.john[self.n as usize - 1] as usize];
        self.ann.push(ann);
        self.john.push(john);
        self.n += 1;
        return Some((ann, john))
    }
}

fn john(n: i32) -> Vec<i32> {
    let mut r = AnnJohn::new();
    for _ in 0..n {r.next();}
    r.john
}

fn ann(n: i32) -> Vec<i32> {
    let mut r = AnnJohn::new();
    for _ in 0..n {r.next();}
    r.ann
}

fn sum_john(n: i32) -> i32 {
    let mut r = AnnJohn::new();
    for _ in 0..n {r.next();}
    r.john.iter().sum()
}

fn sum_ann(n: i32) -> i32 {
    let mut r = AnnJohn::new();
    for _ in 0..n {r.next();}
    r.ann.iter().sum()
}

pub fn all(n: i32) {
    println!("ann {}: {:?}", n, ann(n));
    println!("john {}: {:?}", n, john(n));
    println!("sum ann {}: {:?}", n, sum_ann(n));
    println!("sum john {}: {:?}", n, sum_john(n));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_john() {
        assert_eq!(john(11), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
        assert_eq!(john(14), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
    }
    #[test]
    fn test_ann() {
        assert_eq!(ann(6), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(ann(15), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
    }
    #[test]
    fn test_sum_john() {
        assert_eq!(sum_john(75), 1720);
        assert_eq!(sum_john(78), 1861);
    }
    #[test]
    fn test_sum_ann() {
        assert_eq!(sum_ann(115), 4070);
        assert_eq!(sum_ann(150), 6930);
    }
}
