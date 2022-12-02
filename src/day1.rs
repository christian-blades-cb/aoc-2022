pub fn part1(xs: &str) -> usize {
    let mut max = 0;
    let mut cur = 0;
    for ln in xs.lines() {
        if ln == "" {
            max = cur.max(max);
            cur = 0;
        } else {
            cur += ln.parse::<usize>().unwrap();
        }
    }
    max = cur.max(max);
    max
}

pub fn part2(xs: &str) -> usize {
    let mut calories = Vec::new();
    let mut cur = 0;
    for ln in xs.lines() {
        if ln == "" {
            calories.push(cur);
            cur = 0;
        } else {
            cur += ln.parse::<usize>().unwrap();
        }
    }
    calories.push(cur);
    calories.sort();
    calories.reverse();
    calories.into_iter().take(3).sum()
}
