#![feature(vec_retain_mut)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let (number_order, boards) = input.split_once("\n\n").unwrap();

    let number_order = number_order
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|val| val.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", last_bingo_score(&number_order, &boards));
}

fn last_bingo_score(number_order: &[u32], boards: &[Vec<Vec<u32>>]) -> u32 {
    let mut counter = vec![(vec![5; 5], vec![5; 5]); boards.len()];
    let mut boards = boards
        .iter()
        .enumerate()
        .map(|(idx, board)| {
            (
                idx,
                board
                    .iter()
                    .enumerate()
                    .flat_map(|(i, items)| {
                        items.iter().enumerate().map(move |(j, &val)| (val, (i, j)))
                    })
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut result = None;
    for num in number_order {
        if boards.is_empty() {
            break;
        }

        boards.retain_mut(|(idx, board)| {
            if let Some((i, j)) = board.remove(num) {
                let (row_count, col_count) = &mut counter[*idx];
                row_count[i] -= 1;
                col_count[j] -= 1;

                if row_count[i] <= 0 || col_count[j] <= 0 {
                    let sum = board.iter().map(|(&val, _)| val).sum::<u32>();
                    result = Some(sum * num);
                    return false;
                }
            }

            true
        });
    }

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::last_bingo_score;

    #[test]
    fn test_first_bingo_score() {
        let number_order = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ],
            vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        ];

        assert_eq!(last_bingo_score(&number_order, &boards), 1924);
    }
}
