use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let arr: Vec<(Point, Point)> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(p1, p2)| (p1.split_once(',').unwrap(), p2.split_once(',').unwrap()))
        .map(|((x1, y1), (x2, y2))| {
            (
                Point {
                    x: x1.parse().unwrap(),
                    y: y1.parse().unwrap(),
                },
                Point {
                    x: x2.parse().unwrap(),
                    y: y2.parse().unwrap(),
                },
            )
        })
        .collect();

    println!("{}", count_overlap_points(&arr));
}

fn count_overlap_points(arr: &[(Point, Point)]) -> usize {
    let mut point_freq: HashMap<Point, u32> = HashMap::new();

    for (p1, p2) in arr {
        let (p1, p2) = if p1.x == p2.x {
            if p1.y > p2.y {
                (p2, p1)
            } else {
                (p1, p2)
            }
        } else if p1.y == p2.y {
            if p1.x > p2.x {
                (p2, p1)
            } else {
                (p1, p2)
            }
        } else {
            continue;
        };

        for i in p1.x..=p2.x {
            for j in p1.y..=p2.y {
                let val = point_freq.entry(Point { x: i, y: j }).or_insert(0);
                *val += 1;
            }
        }
    }

    point_freq.iter().filter(|(_k, &v)| v >= 2).count()
}
