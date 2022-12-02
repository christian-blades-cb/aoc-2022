mod day1;
mod day2;

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
}
