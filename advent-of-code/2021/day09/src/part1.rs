fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let map: Vec<&str> = input.lines().collect();
    let adjacents = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, val) in row.bytes().enumerate() {
            if adjacents.into_iter().all(|(dx, dy)| {
                map.get((i as isize + dx) as usize)
                    .and_then(|row| row.as_bytes().get((j as isize + dy) as usize))
                    .map(|&cell| val < cell)
                    .unwrap_or(true)
            }) {
                sum += (val - b'0') as usize + 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::solve;

    const INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), 15);
    }
}
