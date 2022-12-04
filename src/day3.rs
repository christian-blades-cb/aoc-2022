use std::collections::{HashMap, HashSet};

pub fn part1(xs: &str) -> usize {
    let priorities: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();
    // dbg!(&priorities);
    let mut sum = 0;
    for ln in xs.lines() {
        if ln == "" {
            continue;
        }
        let half = ln.len() / 2;
        let (a, b) = ln.split_at(half);
        let sa: HashSet<char> = a.chars().collect();
        let sb: HashSet<char> = b.chars().collect();
        let intersection = sa.intersection(&sb).next().unwrap();
        sum += priorities.get(intersection).cloned().unwrap();
    }
    sum
}

pub fn part2(xs: &str) -> usize {
    let priorities: HashMap<char, usize> = ('a'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();
    // dbg!(&priorities);
    let mut sum = 0;
    let mut lines = xs.lines().filter(|x| x.len() != 0);
    while let Ok(chunk) = lines.next_chunk::<3>() {
        let a: HashSet<char> = chunk[0].chars().collect();
        let b: HashSet<char> = chunk[1].chars().collect();
        let c: HashSet<char> = chunk[2].chars().collect();
        let i1: HashSet<char> = a.intersection(&b).cloned().collect();
        let i = c.intersection(&i1).next().unwrap();
        sum += priorities.get(i).unwrap();
    }
    sum
}
