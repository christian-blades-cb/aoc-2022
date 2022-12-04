use std::cmp::Ordering;

pub fn part1(xs: &str) -> usize {
    let mut sum = 0;
    for line in xs.lines().filter(|x| x.len() != 0) {
        let (a, b) = line.split_once(',').unwrap();
        let (ax, ay) = {
            let (x, y) = a.split_once('-').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        };
        let (bx, by) = {
            let (x, y) = b.split_once('-').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        };

        sum += match (ax.cmp(&bx), ay.cmp(&by)) {
            (Ordering::Less, Ordering::Greater) => 1,
            (Ordering::Less, Ordering::Equal) => 1,
            (Ordering::Equal, Ordering::Equal) => 1,
            (Ordering::Equal, Ordering::Greater) => 1,

            (Ordering::Greater, Ordering::Less) => 1,
            (Ordering::Equal, Ordering::Less) => 1,
            (Ordering::Greater, Ordering::Equal) => 1,

            _ => 0,
        }
    }
    sum
}

pub fn part2(xs: &str) -> usize {
    let mut sum = 0;
    for line in xs.lines().filter(|x| x.len() != 0) {
        let (a, b) = line.split_once(',').unwrap();
        let (ax, ay) = {
            let (x, y) = a.split_once('-').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        };
        let (bx, by) = {
            let (x, y) = b.split_once('-').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        };

        sum += if (ay < bx || ax > by) && (by < ax || bx > ay) {
            0
        } else {
            1
        }
    }
    sum
}
