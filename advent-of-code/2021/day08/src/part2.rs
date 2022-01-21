use std::{collections::HashMap};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(signals, digits)| {
            let signals = signals.split_whitespace().collect::<Vec<_>>();
            let char_to_idx = find_char_to_idx(&signals);

            let value = digits
                .split_whitespace()
                .map(move |digit| find_digit(&char_to_idx, digit.as_bytes()))
                .collect::<Vec<_>>();

            std::str::from_utf8(&value)
                .unwrap().parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn find_char_to_idx(signals: &[&str]) -> HashMap<u8, u8> {
    let char_freq: HashMap<u8, u8> = signals
        .iter()
        .flat_map(|signal| signal.as_bytes().iter())
        .fold(HashMap::new(), |mut accu, &ch| {
            *accu.entry(ch).or_insert(0) += 1;
            accu
        });

    let b = char_freq.iter().find_map(|(k, &v)| if v == 6 {Some(*k)} else {None}).unwrap();
    let e = char_freq.iter().find_map(|(k, &v)| if v == 4 {Some(*k)} else {None}).unwrap();
    let f = char_freq.iter().find_map(|(k, &v)| if v == 9 {Some(*k)} else {None}).unwrap();

    let one = *signals.iter().find(|signal| signal.len() == 2).unwrap();
    let c = one.bytes().find(|&ch| ch != e && ch != f).unwrap();

    let seven = *signals.iter().find(|signal| signal.len() == 3).unwrap();
    let a = seven.bytes().find(|&ch| ch != c && ch != f).unwrap();

    let four = *signals.iter().find(|signal| signal.len() == 4).unwrap();
    let d = four
        .bytes()
        .find(|&ch| ch != b && ch != c && ch != f)
        .unwrap();

    let eight = *signals.iter().find(|signal| signal.len() == 7).unwrap();
    let g = eight
        .bytes()
        .find(|&ch| ch != a && ch != b && ch != c && ch != d && ch != e && ch != f)
        .unwrap();

    [a, b, c, d, e, f, g]
        .into_iter()
        .zip((1..=7).rev())
        .collect()
}

fn find_digit(char_to_idx: &HashMap<u8, u8>, seg_chars: &[u8]) -> u8 {
    let mut val: u8 = 0;
    for ch in seg_chars {
        // one based index
        if let Some(&idx) = char_to_idx.get(ch) {
            val |= 1 << (idx - 1);
        } else {
            unreachable!()
        }
    }

    match val {
        //abcdefg
        0b1110111 => b'0',
        0b0010010 => b'1',
        0b1011101 => b'2',
        0b1011011 => b'3',
        0b0111010 => b'4',
        0b1101011 => b'5',
        0b1101111 => b'6',
        0b1010010 => b'7',
        0b1111111 => b'8',
        0b1111011 => b'9',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_digit, solve};

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(
                "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"
            ),
            61229
        );
    }

    #[test]
    fn test_find_digit() {
        assert_eq!(
            find_digit(&(b'a'..=b'g').zip((1..=7).rev()).collect(), &[b'c', b'f']),
            1
        );

        assert_eq!(
            find_digit(
                &(b'a'..=b'g').zip((1..=7).rev()).collect(),
                &[b'a', b'b', b'd', b'f', b'g']
            ),
            5
        );
    }
}
