use std::{cmp::max, ops::RangeInclusive};

fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let (x_range, y_range) = input
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .map(|(x, y)| (x.split_once("..").unwrap(), y.split_once("..").unwrap()))
        .map(|((x1, x2), (y1, y2))| {
            (
                x1.parse::<i32>().unwrap()..=x2.parse::<i32>().unwrap(),
                y1.parse::<i32>().unwrap()..=y2.parse::<i32>().unwrap(),
            )
        })
        .unwrap();

    let mut max_height = 0;

    for dx in 0..500 {
        for dy in 0..500 {
            let mut projectile = Projectile::new(dx, dy, x_range.clone(), y_range.clone());
            while projectile.next() {}
            if projectile.passes_target_area() {
                max_height = max(max_height, projectile.max_height());
            }
        }
    }

    max_height as usize
}

struct Projectile {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    max_y: i32,
    passes_target_area: bool,
}

impl Projectile {
    fn new(dx: i32, dy: i32, x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
        Projectile {
            x: 0,
            y: 0,
            dx,
            dy,
            x_range,
            y_range,
            max_y: 0,
            passes_target_area: false,
        }
    }

    fn passes_target_area(&self) -> bool {
        self.passes_target_area
    }

    fn max_height(&self) -> i32 {
        self.max_y
    }

    fn next(&mut self) -> bool {
        self.x += self.dx;
        self.y += self.dy;

        self.dy -= 1;
        if self.dx > 0 {
            self.dx -= 1;
        }

        self.max_y = max(self.max_y, self.y);
        if !self.passes_target_area {
            self.passes_target_area =
                self.x_range.contains(&self.x) && self.y_range.contains(&self.y)
        }

        self.y >= *self.y_range.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("target area: x=20..30, y=-10..-5"), 45);
    }
}
