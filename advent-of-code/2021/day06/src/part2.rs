use std::collections::HashMap;

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let fishes: Vec<i32> = input.split(',').map(|val| val.parse().unwrap()).collect();
    let mut cache = HashMap::new();
    fishes
        .iter()
        .map(|&f| count_offsprings(f, 256, &mut cache))
        .sum()
}

fn count_offsprings(timer: i32, days: i32, cache: &mut HashMap<(i32, i32), usize>) -> usize {
    if let Some(&value) = cache.get(&(timer, days)) {
        return value;
    }

    let mut count = 1;
    let mut rem_days = days - timer - 1;
    while rem_days >= 0 {
        count += count_offsprings(8, rem_days, cache);
        rem_days -= 7;
    }
    cache.insert((timer, days), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("3,4,3,1,2"), 26984457539);
    }

    #[test]
    fn test_count_offsprings() {
        assert_eq!(count_offsprings(3, 20, &mut HashMap::new()), 7);
    }
}
