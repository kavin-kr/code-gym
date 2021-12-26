fn main() {
    let arr = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let count = arr.windows(2)
        .filter(|slice| slice[0] < slice[1])
        .count();

    println!("{}", count);
}
