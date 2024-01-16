use regex::Regex;
use itertools::Itertools;

struct Address {
    street: String,
    number: u32,
}

pub fn travel(r: &str, zipcode:&str) -> String {
    let formatted = format!(r"(?P<house_number>\d+) (?P<street>.+) {}$", zipcode);
    let re = Regex::new(formatted.as_str()).unwrap();
    let result: Vec<Address> = r
        .split(",")
        .filter_map(|m| re.captures(m))
        .map(|m| Address{street: m["street"].to_string(), number: m["house_number"].parse::<u32>().unwrap()})
        .collect();
    format!("{}:{}/{}", zipcode, result.iter().map(|a| &a.street).join(","), result.iter().map(|a| a.number).join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ad() -> String {
        return String::from("123 Main Street St. Louisville OH 43071, 432 Main Long Road St. Louisville OH 43071,786 High Street Pollocksville NY 56432,
        54 Holy Grail Street Niagara Town ZP 32908, 3200 Main Rd. Bern AE 56210,1 Gordon St. Atlanta RE 13000,
        10 Pussy Cat Rd. Chicago EX 34342, 10 Gordon St. Atlanta RE 13000, 58 Gordon Road Atlanta RE 13000,
        22 Tokyo Av. Tedmondville SW 43098, 674 Paris bd. Abbeville AA 45521, 10 Surta Alley Goodtown GG 30654,
        45 Holy Grail Al. Niagara Town ZP 32908, 320 Main Al. Bern AE 56210, 14 Gordon Park Atlanta RE 13000,
        100 Pussy Cat Rd. Chicago EX 34342, 2 Gordon St. Atlanta RE 13000, 5 Gordon Road Atlanta RE 13000,
        2200 Tokyo Av. Tedmondville SW 43098, 67 Paris St. Abbeville AA 45521, 11 Surta Avenue Goodtown GG 30654,
        45 Holy Grail Al. Niagara Town ZP 32918, 320 Main Al. Bern AE 56215, 14 Gordon Park Atlanta RE 13200,
        100 Pussy Cat Rd. Chicago EX 34345, 2 Gordon St. Atlanta RE 13222, 5 Gordon Road Atlanta RE 13001,
        2200 Tokyo Av. Tedmondville SW 43198, 67 Paris St. Abbeville AA 45522, 11 Surta Avenue Goodville GG 30655,
        2222 Tokyo Av. Tedmondville SW 43198, 670 Paris St. Abbeville AA 45522, 114 Surta Avenue Goodville GG 30655,
        2 Holy Grail Street Niagara Town ZP 32908, 3 Main Rd. Bern AE 56210, 77 Gordon St. Atlanta RE 13000,
        100 Pussy Cat Rd. Chicago OH 13201");
    }

    fn dotest(r: &str, zipcode:&str, exp: &str) -> () {
        //println!("r:{:?}", r);
        println!(" zipcode:{:?}", zipcode);
        let ans = travel(r, zipcode);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        let r = &ad();
        dotest(r, "AA 45522", "AA 45522:Paris St. Abbeville,Paris St. Abbeville/67,670");
        dotest(r, "OH 430", "OH 430:/");
    }
}
