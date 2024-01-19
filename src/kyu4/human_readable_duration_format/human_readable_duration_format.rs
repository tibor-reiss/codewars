const MINUTE: u64 = 60;
const HOUR: u64 = MINUTE * 60;
const DAY: u64 = HOUR * 24;
const YEAR: u64 = DAY * 365;

fn format_one(number: u64, unit: &str) -> Option<String> {
    match number {
        0 => None,
        1 => Some(format!("{} {}", number, unit)),
        _ => Some(format!("{} {}s", number, unit)),
    }
}

pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {return String::from("now")};

    let (year, seconds) = (seconds / YEAR, seconds % YEAR);
    let (day, seconds) = (seconds / DAY, seconds % DAY);
    let (hour, seconds) = (seconds / HOUR, seconds % HOUR);
    let (minute, seconds) = (seconds / MINUTE, seconds % MINUTE);
    
    let mut result = Vec::new();
    for (number, unit) in [
        (year, "year"),
        (day, "day"),
        (hour, "hour"),
        (minute, "minute"),
        (seconds, "second")
     ] {
        match format_one(number, unit) {
            Some(number) => result.push(number),
            None => (),
        }
    }

    match result.len() {
        0 => String::from("now"),
        1 => result[0].clone(),
        _ => {
            let (last, elements) = result.split_last().unwrap();
            elements.join(", ") + &format!(" and {last}")
        },
    }
}

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
