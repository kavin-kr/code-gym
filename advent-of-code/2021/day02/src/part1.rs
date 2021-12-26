fn main() {
    let (pos, dep) = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .fold((0, 0), |(pos, dep), (cmd, val)| {
            let val = val.parse::<i32>().unwrap();
            match cmd {
                "forward" => (pos + val, dep),
                "down" => (pos, dep + val),
                "up" => (pos, dep - val),
                _ => unreachable!(),
            }
        });

    println!("{}", pos * dep);
}
