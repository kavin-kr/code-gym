use std::{cmp::Ordering, collections::BTreeSet};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let (dots, inst) = input
        .split_once("\n\n")
        .map(|(dots, inst)| {
            (
                dots.lines()
                    .map(|l| l.split_once(',').unwrap())
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .collect::<BTreeSet<(usize, usize)>>(),
                inst.lines()
                    .map(|l| l.split_once('=').unwrap())
                    .map(|(dir, val)| (dir.chars().last().unwrap(), val.parse().unwrap()))
                    .collect::<Vec<(char, usize)>>(),
            )
        })
        .unwrap();

    inst.iter()
        .take(1)
        .fold(dots, |dots, &(dir, val)| {
            dots.iter()
                .filter_map(|&(x, y)| match dir {
                    'x' => match x.cmp(&val) {
                        Ordering::Less => Some((x, y)),
                        Ordering::Greater => Some((val - (x - val), y)),
                        Ordering::Equal => None,
                    },
                    'y' => match y.cmp(&val) {
                        Ordering::Less => Some((x, y)),
                        Ordering::Greater => Some((x, val - (y - val))),
                        Ordering::Equal => None,
                    },
                    _ => unreachable!(),
                })
                .collect()
        })
        .len()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(solve(input), 17);
    }
}
