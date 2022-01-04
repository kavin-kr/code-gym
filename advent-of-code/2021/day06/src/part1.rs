fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let mut arr: Vec<i32> = input.split(',').map(|val| val.parse().unwrap()).collect();

    for _day in 1..=80 {
        let new_fish = arr
            .iter_mut()
            .filter_map(|timer| {
                if *timer == 0 {
                    *timer = 6;
                    Some(())
                } else {
                    *timer -= 1;
                    None
                }
            })
            .count();
        arr.append(&mut vec![8; new_fish]);
    }

    arr.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("3,4,3,1,2"), 5934);
    }
}
