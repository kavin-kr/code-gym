fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    input.lines().map(find_first_corrupted).fold(0, |accu, x| {
        accu + match x {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        }
    })
}

fn find_first_corrupted(line: &str) -> Option<char> {
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
                return Some(b);
            }
        }
    }

    None
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

        assert_eq!(solve(input), 26397);
    }
}
