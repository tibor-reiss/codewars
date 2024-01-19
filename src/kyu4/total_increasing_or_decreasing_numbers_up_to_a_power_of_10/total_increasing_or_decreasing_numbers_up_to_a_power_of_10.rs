fn inc(n: u32) -> u64 {
    //First dimension: number of digits
    //Second dimension: starting digit
    //for non-0: itself + everything from before which was starting with a higher number
    let mut result: Vec<Vec<u64>> = vec![];
    result.push(vec![1; 10]);  // n==1
    for i in 1..n as usize {
        let mut row: Vec<u64> = vec![];
        for j in 0..10 {
            match j {
                0 => row.push(0),
                _ => row.push(result[i-1][j..10].iter().sum::<u64>()),
            }
            //println!("{}", row[j]);
        }
        //println!("********");
        result.push(row);
    }
    result.iter().map(|row| row.iter().sum::<u64>()).sum()
}

fn dec(n: u32) -> u64 {
    //First dimension: number of digits
    //Second dimension: starting digit
    //for non-0: itself + everything from before which was starting with a lower number + 1
    let mut result: Vec<Vec<u64>> = vec![];
    result.push(vec![1; 10]);  // n==1
    for i in 1..n as usize{
        let mut row: Vec<u64> = vec![];
        for j in 0..10 {
            match j {
                0 => row.push(0),
                _ => {
                    //+1 because it gets a zero
                    row.push(result[i-1][1..j+1].iter().sum::<u64>() + 1);
                },
            }
            //println!("{}", row[j]);
        }
        //println!("********");
        result.push(row);
    }
    //Remove n==1 and numbers which only have one digit
    result.iter().map(|row| row.iter().sum::<u64>()).sum::<u64>() - 10 - (n as u64 - 1) * 9
}

pub fn total_inc_dec(n: u32) -> u64 {
    if n == 0 {return 1};
    let i = inc(n);
    let d = dec(n);
    //println!("{} {}", i, d);
    i + d
}

#[cfg(test)]
mod tests {
    use super::total_inc_dec;
        
    fn dotest(n: u32, expected: u64) {
        let actual = total_inc_dec(n);
        assert!(actual == expected, "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, 1);
        dotest(1, 10);
        dotest(2, 100);
        dotest(3, 475);
        dotest(4, 1675);
    }
}
