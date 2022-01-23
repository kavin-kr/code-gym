use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let adj_list = input.lines().map(|l| l.split_once('-').unwrap()).fold(
        HashMap::new(),
        |mut accu, (from, to)| {
            accu.entry(from).or_insert(vec![]).push(to);
            accu.entry(to).or_insert(vec![]).push(from);
            accu
        },
    );

    dfs("start", &mut HashSet::from(["start"]), false, &adj_list)
}

fn dfs<'a>(
    from: &'a str,
    visited: &mut HashSet<&'a str>,
    used_special: bool,
    adj_list: &'a HashMap<&str, Vec<&str>>,
) -> usize {
    if from == "end" {
        return 1;
    }

    adj_list.get(from).map_or(0, |tos| {
        tos.iter().fold(0, |mut accu, to| {
            if to.chars().all(|ch| ch.is_uppercase()) {
                let fresh_insert = visited.insert(to);
                accu += dfs(to, visited, used_special, adj_list);
                if fresh_insert {
                    visited.remove(to);
                }
            } else if to != &"start" {
                if !visited.contains(to) {
                    visited.insert(to);
                    accu += dfs(to, visited, used_special, adj_list);
                    visited.remove(to);
                } else if !used_special {
                    accu += dfs(to, visited, true, adj_list);
                }
            }
            accu
        })
    })
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(solve(input), 36);
    }
}
