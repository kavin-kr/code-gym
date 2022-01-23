use std::collections::VecDeque;

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|&b| b - b'0').collect())
        .collect();

    (0..100)
        .map(|_| flash_count_after_iteration(&mut grid))
        .sum()
}

fn flash_count_after_iteration(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut queue = VecDeque::new();
    for (i, inner) in grid.iter_mut().enumerate() {
        for (j, val) in inner.iter_mut().enumerate() {
            *val += 1;
            if *val > 9 {
                queue.push_back((i, j));
            }
        }
    }

    const ADJACENTS: [(isize, isize); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    while let Some((i, j)) = queue.pop_front() {
        ADJACENTS
            .iter()
            .filter_map(|&(di, dj)| {
                let ii = i as isize + di;
                let jj = j as isize + dj;

                if (0..10).contains(&ii) && (0..10).contains(&jj) {
                    Some((ii as usize, jj as usize))
                } else {
                    None
                }
            })
            .for_each(|(ii, jj)| {
                if grid[ii][jj] <= 9 {
                    grid[ii][jj] += 1;
                    if grid[ii][jj] == 10 {
                        queue.push_back((ii, jj));
                    }
                }
            })
    }

    let mut count = 0;
    for inner in grid.iter_mut() {
        for val in inner.iter_mut() {
            if *val == 10 {
                *val = 0;
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(solve(input), 1656);
    }
}
