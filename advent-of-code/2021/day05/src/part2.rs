#![feature(int_abs_diff)]
use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    type Point = (i32, i32);
    let line_segments: Vec<(Point, Point)> = input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(p1, p2)| (p1.split_once(',').unwrap(), p2.split_once(',').unwrap()))
        .map(|((x1, y1), (x2, y2))| {
            (
                (x1.parse().unwrap(), y1.parse().unwrap()),
                (x2.parse().unwrap(), y2.parse().unwrap()),
            )
        })
        .collect();

    let mut point_freq: HashMap<Point, u32> = HashMap::new();

    for ((x1, y1), (x2, y2)) in line_segments {
        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                *point_freq.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                *point_freq.entry((x, y1)).or_insert(0) += 1;
            }
        } else if (x1 - x2 == y1 - y2) || (x1 - x2 == y2 - y1) {
            let dx = if x1 < x2 { 1 } else { -1 };
            let dy = if y1 < y2 { 1 } else { -1 };
            for i in 0..=i32::abs(x1 - x2) {
                *point_freq
                    .entry((x1 + (dx * i), y1 + (dy * i)))
                    .or_insert(0) += 1;
            }
        } else {
            unimplemented!()
        }
    }

    point_freq.iter().filter(|(_k, &v)| v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
        assert_eq!(solve(input), 12);
    }
}
