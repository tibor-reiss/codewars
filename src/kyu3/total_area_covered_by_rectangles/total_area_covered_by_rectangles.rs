use std::collections::{HashMap,HashSet};
use itertools::Itertools;
use std::cmp::max;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Rect {
    left: i32,
    bottom: i32,
    right: i32,
    top: i32,
}

pub fn calculate(rectangles: &[[i32; 4]]) -> i64 {
    
    if rectangles.is_empty() {
        return 0;
    }
    let mut area = 0_i64;
    let mut starts = HashMap::new();
    let mut ends = HashMap::new();
    let mut xs = HashSet::new();
    for r in rectangles {
        let rect = Rect { left: r[0], bottom: r[1], right: r[2], top: r[3] };
        let left = starts.entry(rect.left).or_insert_with(HashSet::<Rect>::new);
        let right = ends.entry(rect.right).or_insert_with(HashSet::<Rect>::new);
        left.insert(rect);
        right.insert(rect);
        xs.insert(rect.left);
        xs.insert(rect.right);
    }
    let xs_sorted = xs.into_iter().sorted().collect::<Vec<i32>>();
    let mut working_set = HashSet::new();
    for i in 0..xs_sorted.len() - 1 {
        let (x0, x1) = (xs_sorted[i], xs_sorted[i+1]);
        if let Some(start_rects) = starts.get(&x0) {
            working_set = &working_set | &start_rects;
        }
        if let Some(end_rects) = ends.get(&x0) {
            working_set = &working_set - &end_rects;
        }
        if !working_set.is_empty() {
            area += get_total_area(x0.into(), x1.into(), &working_set);
        }
    }
    area
}

fn get_total_area(x0: i64, x1: i64, rectangles: &HashSet<Rect>) -> i64 {
    let mut area = 0_i64;
    let rects: Vec<Rect> = rectangles.clone().into_iter().sorted_by_key(|a| a.bottom).collect();
    let mut unions: Vec<[i32; 2]> = Vec::new();
    let mut range = [rects[0].bottom, rects[0].top];
    for rect in rects.iter().skip(1) {
        if rect.bottom <= range[1] {
            range[1] = max(rect.top, range[1]);
        }
        else {
            unions.push(range);
            range = [rect.bottom, rect.top];
        }
    }
    unions.push(range);
    for span in unions {
        area += (x1 - x0) * (span[1] - span[0]) as i64;
    }
    area
}

#[cfg(test)]
mod sample_tests {
    use super::calculate;
    
    const ERR_MSG : &str = "\nYour result (left) did not equal expected result (right)";

    #[test]
    fn zero_rectangles() {
        assert_eq!(calculate(&[]), 0, "{}", ERR_MSG);
    }
    
    #[test]
    fn one_rectangle() {
        assert_eq!(calculate(&[[0,0,1,1]]), 1, "{}", ERR_MSG);
        assert_eq!(calculate(&[[0,4,11,6]]), 22, "{}", ERR_MSG);
    }
    
    #[test]
    fn two_rectangles() {
        assert_eq!(calculate(&[[0,0,1,1], [1,1,2,2]]), 2, "{}", ERR_MSG);
        assert_eq!(calculate(&[[0,0,1,1], [0,0,2,2]]), 4, "{}", ERR_MSG);
    }
    
    #[test]
    fn three_rectangles() {
        assert_eq!(calculate(&[[3,3,8,5], [6,3,8,9], [11,6,14,12]]), 36, "{}", ERR_MSG);
    }
}
