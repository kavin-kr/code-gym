fn main() {
    let arr = include_str!("../input.txt")
        .lines()
        .map(|line| u16::from_str_radix(line, 2).expect("integer string"))
        .collect::<Vec<_>>();

    let o2 = filter_ratings(arr.clone(), true);
    let co2 = filter_ratings(arr, false);

    println!("{}", o2 as u32 * co2 as u32);
}

fn filter_ratings(arr: Vec<u16>, use_most_common_bit: bool) -> u16 {
    let mut arr = arr;
    for idx in (0..12).rev() {
        if arr.len() <= 1 {
            break;
        }

        let retain_bit = {
            let ones_count = arr.iter().filter(|&&val| (val & 1 << idx) > 0).count();
            let most_common_bit = 2 * ones_count >= arr.len(); // 1 is used if both 0 & 1 are equal
            if use_most_common_bit {
                most_common_bit
            } else {
                !most_common_bit
            }
        };

        arr.retain(|item| ((item & 1 << idx) > 0) == retain_bit);
    }

    if arr.len() != 1 {
        panic!("After filtering: arr.len() = {}, it should be exactly 1", arr.len())
    }
    arr.first().copied().unwrap()
}
