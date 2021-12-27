use std::cmp::Ordering;

fn main() {
    let arr = include_str!("../input.txt")
        .lines()
        .fold(vec![0; 12], |mut accu, line| {
            for (idx, ch) in line.bytes().enumerate() {
                match ch {
                    b'0' => accu[idx] -= 1,
                    b'1' => accu[idx] += 1,
                    _ => unreachable!(),
                }
            }
            accu
        });

    let (mut gamma, mut epsilon) = (0, 0);
    for (idx, val) in arr.iter().rev().enumerate() {
        match val.cmp(&0) {
            Ordering::Less => epsilon |= 1 << idx,
            Ordering::Greater => gamma |= 1 << idx,
            Ordering::Equal => unreachable!(),
        }
    }

    println!("{}", gamma * epsilon);
}
