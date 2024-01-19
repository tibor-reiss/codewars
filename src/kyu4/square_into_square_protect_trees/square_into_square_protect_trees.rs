use itertools::Itertools;

pub fn decompose(n: i64) -> Option<Vec<i64>> {
    if n == 0 {return None};
    
    let mut s: Vec<i64> = vec![];
    decompose_recursive(n * n, n * n, &mut s);
    match s.len() {
        0 => None,
        _ => Some(s.into_iter().rev().collect_vec()),
    }
}

fn decompose_recursive(sum: i64, n: i64, s: &mut Vec<i64>) {
    if n == 0 {return};
    let mut i: i64 = f64::sqrt(n as f64).round() as i64;
    if i * i > n || (i*i == n && n == sum) {i -= 1};
    println!("{}", i);
    
    loop {
        if s.iter().map(|v| v*v).sum::<i64>() == sum {break};

        if 2 * i * i <= n || i == 0 {
            s.pop();
            break;
        }

        if !s.contains(&i) {
            s.push(i);
            decompose_recursive(sum, n - i * i, s);
        }

        i -= 1;
    }
}

fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1,3,5,8,49]));
    testing(44, Some(vec![2,3,5,7,43]));
    testing(625, Some(vec![2,5,8,34,624]));
    testing(5, Some(vec![3,4]));
    
}
