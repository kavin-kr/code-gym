fn main() {
    let (pos, dep, _) = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .fold((0, 0, 0), |(pos, dep, aim), (cmd, val)| {
            let val = val.parse::<i32>().unwrap();
            match cmd {
                "forward" => (pos + val, dep + (aim * val), aim),
                "down" => (pos, dep, aim + val),
                "up" => (pos, dep, aim - val),
                _ => unreachable!(),
            }
        });

    println!("{}", pos * dep);
}
