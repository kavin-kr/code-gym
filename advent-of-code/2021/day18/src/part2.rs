use std::cmp::max;

use snailfish::Num;

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let arr = input.lines().map(Num::from_str).collect::<Vec<_>>();

    let mut max_val = 0;
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i != j {
                max_val = max(
                    max_val,
                    Num::add(arr[i].clone(), arr[j].clone()).magnitude(),
                );
            }
        }
    }
    max_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(solve(input), 3993);
    }
}

mod snailfish {
    use std::fmt::Display;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Num {
        left: Box<Node>,
        right: Box<Node>,
    }

    impl Num {
        fn new(left: Node, right: Node) -> Self {
            Num {
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        pub fn from_str(num: &str) -> Self {
            assert!(num.starts_with('[') && num.ends_with(']'));
            let num = &num[1..num.len() - 1];

            let comma_idx = || -> usize {
                let mut brackets_count = 0;
                for (idx, ch) in num.bytes().enumerate() {
                    if ch == b'[' {
                        brackets_count += 1;
                    } else if ch == b']' {
                        brackets_count -= 1;
                    } else if ch == b',' && brackets_count == 0 {
                        return idx;
                    }
                }
                unreachable!()
            }();

            Num::new(
                Node::from_str(&num[..comma_idx]),
                Node::from_str(&num[comma_idx + 1..]),
            )
        }

        fn split(&mut self) -> bool {
            let mut stack = vec![self.right.as_mut(), self.left.as_mut()];
            while let Some(node) = stack.pop() {
                match node {
                    Node::Int(val) => {
                        if *val > 9 {
                            *node = Node::Pair(Num::new(
                                Node::Int(*val / 2),
                                Node::Int((*val + 1) / 2),
                            ));
                            return true;
                        }
                    }
                    Node::Pair(num) => {
                        stack.push(num.right.as_mut());
                        stack.push(num.left.as_mut());
                    }
                }
            }
            false
        }

        fn explode(&mut self) -> bool {
            let mut stack = vec![(self.right.as_mut(), 1), (self.left.as_mut(), 1)];
            let mut prev: Option<&mut u32> = None;

            while let Some((node, depth)) = stack.pop() {
                if depth == 4 {
                    if let Some((&l_val, &r_val)) = node
                        .get_num()
                        .and_then(|num| num.left.get_int().zip(num.right.get_int()))
                    {
                        *node = Node::Int(0);

                        if let Some(val) = prev {
                            *val += l_val;
                        }
                        while let Some((node, depth)) = stack.pop() {
                            match node {
                                Node::Int(val) => {
                                    *val += r_val;
                                    break;
                                }
                                Node::Pair(num) => {
                                    stack.push((num.right.as_mut(), depth + 1));
                                    stack.push((num.left.as_mut(), depth + 1));
                                }
                            }
                        }
                        return true;
                    }
                }

                match node {
                    Node::Int(val) => prev = Some(val),
                    Node::Pair(num) => {
                        stack.push((num.right.as_mut(), depth + 1));
                        stack.push((num.left.as_mut(), depth + 1));
                    }
                }
            }
            false
        }

        fn reduce(&mut self) {
            // println!("BEFORE: {}", self);
            while self.explode() || self.split() {
                // println!("    {}", self);
            }
            // println!("AFTER: {}", self);
        }

        pub fn add(first: Num, second: Num) -> Self {
            let mut num = Num {
                left: Box::new(Node::Pair(first)),
                right: Box::new(Node::Pair(second)),
            };
            num.reduce();
            num
        }

        pub fn magnitude(&self) -> usize {
            (3 * self.left.magnitude()) + (2 * self.right.magnitude())
        }
    }

    impl Display for Num {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{},{}]", self.left, self.right)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Node {
        Int(u32),
        Pair(Num),
    }

    impl Node {
        fn from_str(num: &str) -> Self {
            if num.starts_with('[') {
                Node::Pair(Num::from_str(num))
            } else {
                Node::Int(num.parse().unwrap())
            }
        }

        fn magnitude(&self) -> usize {
            match self {
                Node::Int(val) => *val as usize,

                Node::Pair(val) => val.magnitude(),
            }
        }

        fn get_int(&self) -> Option<&u32> {
            match self {
                Node::Int(val) => Some(val),
                Node::Pair(_) => None,
            }
        }

        fn get_num(&self) -> Option<&Num> {
            match self {
                Node::Int(_) => None,
                Node::Pair(val) => Some(val),
            }
        }
    }

    impl Display for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node::Int(val) => write!(f, "{}", val),
                Node::Pair(val) => write!(f, "{}", val),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::snailfish::Num;

        #[test]
        fn test_num_parse_and_reduce() {
            let mut num = Num::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
            num.reduce();
            assert_eq!(num.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        }

        #[test]
        fn test_magnitude() {
            assert_eq!(
                Num::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
                3488
            );
        }
    }
}
