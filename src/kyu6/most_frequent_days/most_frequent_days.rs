use chrono::*;

pub fn most_frequent_days(year: i32) -> Vec<String> {
    let mut counter = vec![0; 7];
    let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    for d in chrono::naive::NaiveDate::from_ymd_opt(year, 1, 1).unwrap().iter_days() {
        if d.year() != year {break};
        counter[d.weekday().number_from_monday() as usize - 1] += 1;
    }
    let max = counter.iter().max().unwrap();
    counter
        .iter()
        .enumerate()
        .filter(|(_, value)| *value == max)
        .map(|(idx, _)| days[idx].to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::most_frequent_days;

    #[test]
    fn basic() {
        assert_eq!(most_frequent_days(1770), ["Monday"]);
        assert_eq!(most_frequent_days(1785), ["Saturday"]);
        assert_eq!(most_frequent_days(1794), ["Wednesday"]);
        assert_eq!(most_frequent_days(1901), ["Tuesday"]);
        assert_eq!(most_frequent_days(1910), ["Saturday"]);
        assert_eq!(most_frequent_days(1968), ["Monday", "Tuesday"]);
        assert_eq!(most_frequent_days(1984), ["Monday", "Sunday"]);
        assert_eq!(most_frequent_days(1986), ["Wednesday"]);
        assert_eq!(most_frequent_days(2001), ["Monday"]);
        assert_eq!(most_frequent_days(2016), ["Friday", "Saturday"]);
        assert_eq!(most_frequent_days(2135), ["Saturday"]);
        assert_eq!(most_frequent_days(3043), ["Sunday"]);
        assert_eq!(most_frequent_days(3150), ["Sunday"]);
        assert_eq!(most_frequent_days(3230), ["Tuesday"]);
        assert_eq!(most_frequent_days(3361), ["Thursday"]);
        assert_eq!(most_frequent_days(2000), ["Saturday", "Sunday"]);
        assert_eq!(most_frequent_days(1984), ["Monday", "Sunday"]);
    }
}

