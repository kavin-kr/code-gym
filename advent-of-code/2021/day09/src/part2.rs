fn main() {
    println!("{}", solve(include_bytes!("../input.txt")));
}

fn solve(input: &[u8]) -> usize {
    let mut grid = input
        .split(|&b| b == b'\n')
        .map(|l| l.to_owned())
        .collect::<Vec<_>>();

    let mut size = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != b'9' {
                size.push(find_basin_size(&mut grid, i, j));
            }
        }
    }

    size.sort_unstable();
    size.iter().rev().take(3).product()
}

const ADJACENTS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn find_basin_size(grid: &mut Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    grid[i][j] = b'9';
    1 + ADJACENTS
        .iter()
        .map(|&(dx, dy)| ((i as isize + dx) as usize, (j as isize + dy) as usize))
        .fold(0, |accu, (xx, yy)| {
            if let Some(true) = grid.get(xx).and_then(|l| l.get(yy).map(|&b| b < b'9')) {
                accu + find_basin_size(grid, xx, yy)
            } else {
                accu
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(
                "\
2199943210
3987894921
9856789892
8767896789
9899965678"
                    .as_bytes()
            ),
            1134
        );
    }
}
