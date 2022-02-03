use std::collections::BinaryHeap;

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let inc_within_range = |mut b, v| {
        b += v;
        if b > 9 {
            b -= 9;
        }
        b
    };

    let mut graph: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            (0..5)
                .flat_map(|v| line.bytes().map(move |b| inc_within_range(b - b'0', v)))
                .collect()
        })
        .collect();

    graph.append(
        &mut (1..5)
            .flat_map(|v| {
                graph
                    .iter()
                    .map(move |l| l.iter().map(|&b| inc_within_range(b, v)).collect())
            })
            .collect::<Vec<Vec<u8>>>(),
    );

    lowest_total_risk(&graph)
}

fn lowest_total_risk(graph: &[Vec<u8>]) -> usize {
    let (i_len, j_len) = (graph.len(), graph[0].len());

    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![isize::MAX; j_len]; i_len];

    dist[0][0] = 0;
    heap.push((0, (0, 0)));

    while let Some((cost, (i, j))) = heap.pop() {
        let cost = -cost;
        if i == i_len - 1 && j == j_len - 1 {
            return cost as usize;
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

            let new_cost = cost + graph[ii][jj] as isize;
            if new_cost < dist[ii][jj] {
                dist[ii][jj] = new_cost;
                heap.push((-new_cost, (ii, jj)));
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

        assert_eq!(solve(input), 315);
    }
}
