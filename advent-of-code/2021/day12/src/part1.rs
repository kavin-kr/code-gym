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

    dfs("start", &mut HashSet::from(["start"]), &adj_list)
}

fn dfs<'a>(
    from: &'a str,
    visited: &mut HashSet<&'a str>,
    adj_list: &'a HashMap<&str, Vec<&str>>,
) -> usize {
    if from == "end" {
        return 1;
    }

    adj_list.get(from).map_or(0, |tos| {
        tos.iter().fold(0, |mut accu, to| {
            if !visited.contains(to) || to.chars().all(|ch| ch.is_uppercase()) {
                let not_already = visited.insert(to);
                accu += dfs(to, visited, adj_list);
                if not_already {
                    visited.remove(to);
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
        let tc = [
            (
                "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end",
                10,
            ),
            (
                "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
                19,
            ),
            (
                "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
                226,
            ),
        ];

        for (input, output) in tc {
            assert_eq!(solve(input), output);
        }
    }
}
