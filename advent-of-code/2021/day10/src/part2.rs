fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let mut arr: Vec<usize> = input
        .lines()
        .filter_map(find_remaining_chars_score)
        .collect();

    arr.sort_unstable();
    arr[arr.len() / 2]
}

fn find_remaining_chars_score(line: &str) -> Option<usize> {
    let mut stack = vec![];
    for b in line.chars() {
        if matches!(b, '<' | '[' | '{' | '(') {
            stack.push(b);
        } else {
            let matching_open = match b {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                '>' => '<',
                _ => unreachable!(),
            };

            if *stack.last().unwrap() == matching_open {
                stack.pop();
            } else {
                return None;
            }
        }
    }

    let socre = stack.iter().rev().fold(0, |accu, x| {
        (accu * 5)
            + match x {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
    });
    
    Some(socre)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(solve(input), 288957);
    }
}
