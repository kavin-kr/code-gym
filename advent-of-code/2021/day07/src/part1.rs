#![feature(int_abs_diff)]
fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> i32 {
    let mut arr: Vec<i32> = input.split(',').map(|num| num.parse().unwrap()).collect();

    arr.sort_unstable();
    let median = arr[arr.len() / 2];

    arr.iter().map(|&val| (val - median).abs()).sum()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("16,1,2,0,4,2,7,1,2,14"), 37);
    }
}
