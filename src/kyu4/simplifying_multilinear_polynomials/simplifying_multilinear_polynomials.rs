use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

pub fn simplify(polynomial: &str) -> String {
    let mut result: HashMap<String, i32> = HashMap::new();
    let re = Regex::new(r"(?P<number>[0-9]*)(?P<poly>[a-z]+)").unwrap();
    let mut first: bool;
    let mut change: i32;
    let mut number: i32;
    let mut poly = String::from("");

    for i in polynomial.split("+") {
        first = true;
        number = 1;
        for j in i.split("-") {
            if first {
                first = false;
                if j == "" {
                    continue;
                }
                else {
                    change = 1;
                }
            } else {
                change = -1;
            };
            if let Some(caps) = re.captures(j) {
                match caps["number"].parse::<i32>() {
                    Ok(v) => number = v,
                    _ => number = 1,
                };
                poly = caps["poly"].chars().into_iter().sorted().collect();
            }
            result.entry(poly.clone()).and_modify(|counter| *counter += change*number).or_insert(change*number);
        }
    }
    //println!("{:?}", result);
    
    let mut result_vec = result
        .iter()
        .filter(|(_, j)| **j != 0)
        .collect::<Vec<(&String, &i32)>>();
    result_vec.sort_by(
        |a, b|
        a.0.len().cmp(&b.0.len()).then(a.0.cmp(b.0))
    );
    //println!("{:?}", result_vec);

    let result_str = result_vec.iter().map(|(poly, number)| {
        match **number > 0 {
            true => match **number {
                1 => format!("+{}", poly),
                _ => format!("+{}{}", number, poly),
            }
            false => match **number {
                -1 => format!("-{}", poly),
                _ => format!("{}{}", number, poly),
            }
        }
    }).join("");
    //println!("{:?}", result_str);
    match result_str.starts_with("+") {
        true => result_str[1..result_str.len()].to_string(),
        false => result_str,
    }
}

#[cfg(test)]
mod tests {
    use super::simplify;
    
    fn dotest(polynomial: &str, expected: &str) {
        let actual = simplify(polynomial);
        assert!(actual == expected,
            "Test failed with polynomial = \"{polynomial}\"\nExpected \"{expected}\" but got \"{actual}\"");
    }
    
    #[test]
    fn sample_tests() {
        dotest("dc+dcba", "cd+abcd");
        dotest("2xy-yx", "xy");
        dotest("-a+5ab+3a-c-2a", "-c+5ab");
        dotest("-abc+3a+2ac", "3a+2ac-abc");
        dotest("xyz-xz", "-xz+xyz");
        dotest("a+ca-ab", "a-ab+ac");
        dotest("xzy+zby", "byz+xyz");
        dotest("-y+x", "x-y");
        dotest("y-x", "-x+y");
        dotest("3a+b+4ac+bc-ab+3a-cb-a-a", "4a+b-ab+4ac");
        dotest("+n-5hn+7tjhn-4nh-3n-6hnjt+2jhn+9hn", "-2n+2hjn+hjnt");
        dotest("-8fk+5kv-4yk+7kf-qk+yqv-3vqy+4ky+4kf+yvqkf", "3fk-kq+5kv-2qvy+fkqvy");
        dotest("-15cb-12cb-0c+7cb", "-20bc");
        dotest("-12dy+9yzd-9dyz-13y+8y-10yzd-11yd+15yd+9y", "4y-8dy-10dyz");
        dotest("+11x+11x+0xd-12x+5adx+4xd", "10x+4dx+5adx");
        dotest("-0axz-0xz+0axz+0x+4xaz+14x+14zax", "14x+18axz");
    }
}
