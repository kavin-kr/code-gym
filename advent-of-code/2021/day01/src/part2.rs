fn main() {
    let arr = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let count = arr.windows(4)
        .filter(|slice| slice[0] < slice[3])
        .count();

    println!("{}", count);
}
