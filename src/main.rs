#![feature(iter_next_chunk)]

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day1sample = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
    let day1input = include_str!("../inputs/day1");
    dbg!(day1::part1(day1sample));
    dbg!(day1::part1(day1input));
    dbg!(day1::part2(day1sample));
    dbg!(day1::part2(day1input));

    let day2sample = r#"
A Y
B X
C Z
"#;
    let day2input = include_str!("../inputs/day2");
    dbg!(day2::part1(day2sample));
    dbg!(day2::part1(day2input));
    dbg!(day2::part2(day2sample));
    dbg!(day2::part2(day2input));

    let day3sample = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
    let day3input = include_str!("../inputs/day3");
    dbg!(day3::part1(day3sample));
    dbg!(day3::part1(day3input));
    dbg!(day3::part2(day3sample));
    dbg!(day3::part2(day3input));

    let day4sample = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    let day4input = include_str!("../inputs/day4");
    dbg!(day4::part1(day4sample));
    dbg!(day4::part1(day4input));
    dbg!(day4::part2(day4sample));
    dbg!(day4::part2(day4input));
}
