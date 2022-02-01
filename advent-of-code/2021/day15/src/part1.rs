use std::{cmp::Ordering, collections::BinaryHeap};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let graph: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    lowest_total_risk(&graph)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: usize,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.pos.cmp(&self.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn lowest_total_risk(graph: &[Vec<u8>]) -> usize {
    let (i_len, j_len) = (graph.len(), graph[0].len());

    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![usize::MAX; j_len]; i_len];

    dist[0][0] = 0;
    heap.push(State {
        cost: 0,
        pos: (0, 0),
    });

    while let Some(State { cost, pos: (i, j) }) = heap.pop() {
        if i == i_len - 1 && j == j_len - 1 {
            return cost;
        }
        if cost > dist[i][j] {
            continue;
        }

        for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let ii = (di + i as isize) as usize;
            let jj = (dj + j as isize) as usize;
            if ii >= i_len || jj >= j_len {
                continue;
            }

            let new_cost = cost + graph[ii][jj] as usize;
            if new_cost < dist[ii][jj] {
                dist[ii][jj] = new_cost;
                heap.push(State {
                    cost: new_cost,
                    pos: (ii, jj),
                })
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(solve(input), 40);
    }
}
