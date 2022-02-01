#![feature(array_windows)]
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
                template.as_bytes().array_windows().fold(
                    HashMap::<(u8, u8), usize>::new(),
                    |mut accu, &[f, s]| {
                        *accu.entry((f, s)).or_default() += 1;
                        accu
                    },
                ),
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

    let final_template = (0..40).fold(template, |accu, _| {
        let mut temp = HashMap::new();
        for ((l, r), v) in accu {
            let m = rules[&(l, r)];
            *temp.entry((l, m)).or_default() += v;
            *temp.entry((m, r)).or_default() += v;
        }
        temp
    });

    let freq =
        final_template
            .iter()
            .fold(HashMap::<u8, usize>::new(), |mut accu, (&(l, r), &v)| {
                *accu.entry(l).or_default() += v;
                *accu.entry(r).or_default() += v;
                accu
            });

    let (max, min) = freq
        .iter()
        .fold(None, |accu, (_, &val)| {
            let val = (val  + 1) / 2;
            if let Some((max_val, min_val)) = accu {
                Some((max(max_val, val), min(min_val, val)))
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

        assert_eq!(solve(input), 2188189693529);
    }
}
