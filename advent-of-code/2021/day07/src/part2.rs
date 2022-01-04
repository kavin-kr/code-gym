fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> i32 {
    let arr: Vec<i32> = input.split(',').map(|num| num.parse().unwrap()).collect();

    let avg_floor = arr.iter().sum::<i32>() / arr.len() as i32;

    [avg_floor, avg_floor + 1]
        .iter()
        .map(|avg| {
            arr.iter()
                .map(|&val| {
                    let n = (val - avg).abs();
                    (n * (n + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("16,1,2,0,4,2,7,1,2,14"), 168);
        assert_eq!(solve("1,2,5"), 7);
    }
}
