// uncomment the line below if you need the function
// use preloaded::print_table;

use itertools::Itertools;

pub fn build_matches_table(n: u32) -> Vec<Vec<(u32, u32)>> {
    let mut upper_row: Vec<u32> = (n/2+1..n+1).rev().collect_vec();
    let mut lower_row: Vec<u32> = (1..n/2+1).collect_vec();
    let mut result = vec![get_round(&upper_row, &lower_row)];

    for _ in 1..n-1 {
        (upper_row, lower_row) = rotate(upper_row, lower_row);
        result.push(get_round(&upper_row, &lower_row));
    }

    result
}

fn rotate(upper_row: Vec<u32>, lower_row: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
    let mut new_upper_row = upper_row[1..upper_row.len()].to_vec();
    new_upper_row.push(lower_row[lower_row.len() - 1]);
    let mut new_lower_row = vec![1, upper_row[0]];
    new_lower_row.extend_from_slice(&lower_row[1..lower_row.len()-1]);

    (new_upper_row, new_lower_row)
}

fn get_round(upper_row: &Vec<u32>, lower_row: &Vec<u32>) -> Vec<(u32, u32)> {
    lower_row.iter().zip(upper_row.iter()).map(|(&i, &j)| (i, j)).collect_vec()
}

#[cfg(test)]
mod sample_tests {
    use super::build_matches_table;
    use std::collections::HashSet;
    
    #[test]
    fn with_2_teams() {
        let actual = build_matches_table(2);
        assert!(actual.len() == 1, "The table should have 1 round, but yours had {}\n", actual.len());
        assert!(actual[0].len() == 1, "The round should have 1 match, but yours had {}\n", actual[0].len());
        let m = actual[0][0];
        let (a,b) = if m.0 > m.1 {(m.1,m.0)} else {m};
        assert_eq!((a,b), (1,2), "Match should be 1 vs 2")
    }
    
    #[test]
    fn with_4_teams() {
        let actual = build_matches_table(4);
        let mut expected_matches: HashSet<(u32,u32)> = HashSet::from_iter([(1,2), (3,4), (1,3), (2,4), (1,4), (2,3)]);
        let expected_teams = [1,2,3,4];
        let mut seen = HashSet::new();
        assert!(actual.len() == 3, "The table should have 3 rounds, yours had {}\n", actual.len());
        for (mut i, round) in actual.iter().enumerate() {
            i += 1;
            assert!(round.len() == 2, "Round {i}: each round should have 2 matches, yours had {}:\n{round:?}\n", round.len());
            let mut participants = Vec::new();
            for (mut a, mut b) in round {
                if a > b { (a, b) = (b, a); }
                assert!(!seen.contains(&(a,b)), "Round {i}: Match ({a} vs {b}) has already been played!\n");
                seen.insert((a,b));
                assert!(expected_matches.contains(&(a,b)), "Round {i}: ({a} vs {b}) is not a valid match!\n");
                participants.extend_from_slice(&[a,b]);
                expected_matches.remove(&(a,b));
            }
            participants.sort();
            assert!(participants == expected_teams, "Round {i}: Every team must participate in a round.\n{participants:?} should equal {expected_teams:?}\n")
        }
        assert!(expected_matches.is_empty(), "{} matches were not played:\n{expected_matches:?}\n", expected_matches.len());
    }
}

