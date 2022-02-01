#![feature(iter_intersperse)]
use std::{
    cmp::{max, min},
    collections::HashMap,
};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let (template, rules) = input
        .split_once("\n\n")
        .map(|(template, rules)| {
            (
                template.bytes().collect::<Vec<_>>(),
                rules
                    .lines()
                    .map(|line| {
                        let line = line.as_bytes();
                        ((line[0], line[1]), line[6])
                    })
                    .collect::<HashMap<_, _>>(),
            )
        })
        .unwrap();

    let (max, min) = (0..10)
        .fold(template, |accu, _| {
            let mut temp = Vec::new();
            for i in 0..accu.len() {
                temp.push(accu[i]);
                if i < accu.len() - 1 {
                    temp.push(rules[&(accu[i], accu[i + 1])]);
                }
            }
            temp
        })
        .iter()
        .fold(HashMap::new(), |mut accu, &item| {
            *accu.entry(item).or_insert(0) += 1;
            accu
        })
        .iter()
        .fold(None, |accu, (_, &val)| {
            if let Some((xx, yy)) = accu {
                Some((max(xx, val), min(yy, val)))
            } else {
                Some((val, val))
            }
        })
        .unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(solve(input), 1588);
    }
}
