pub fn part1(state: &mut [Vec<char>], xs: &str) -> String {
    let re = regex::Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    for caps in re.captures_iter(xs) {
        // iterations
        let i = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        // -1 because our indexes start at 0 and theirs start at 1
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        for _ in 0..i {
            if let Some(x) = state[from].pop() {
                state[to].push(x);
            }
        }
    }
    state
        .into_iter()
        .map(|x| x.last().cloned().unwrap())
        .collect()
}

pub fn part2(state: &mut [Vec<char>], xs: &str) -> String {
    let re = regex::Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    for caps in re.captures_iter(xs) {
        // iterations
        let i = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        // -1 because our indexes start at 0 and theirs start at 1
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        let mut buf = Vec::new();
        for _ in 0..i {
            if let Some(x) = state[from].pop() {
                buf.push(x);
            }
        }
        (0..i).for_each(|_| state[to].push(buf.pop().unwrap()));
    }

    state
        .into_iter()
        .map(|x| x.last().cloned().unwrap())
        .collect()
}
