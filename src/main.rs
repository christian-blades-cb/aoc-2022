#![feature(iter_next_chunk)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

    let mut day5_sample_state = vec![
        vec!['z', 'n'],      // 1
        vec!['m', 'c', 'd'], // 2
        vec!['p'],
    ]; // 3
    let day5_sample_instructions = r#"
flurbflob

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    dbg!(day5::part1(
        &mut day5_sample_state,
        day5_sample_instructions
    ));
    let mut day5_state = vec![
        vec!['H', 'C', 'R'],                          // 1
        vec!['B', 'J', 'H', 'L', 'S', 'F'],           // 2
        vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],      // 3
        vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],      // 4
        vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'], // 5
        vec!['T', 'H', 'C', 'G'],                     // 6
        vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'], // 7
        vec!['R', 'J', 'Q', 'G', 'C'],                // 8
        vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'], // 9
    ];
    let day5input = include_str!("../inputs/day5");
    dbg!(day5::part1(&mut day5_state, &day5input));
    let mut day5_sample_state = vec![
        vec!['z', 'n'],      // 1
        vec!['m', 'c', 'd'], // 2
        vec!['p'],
    ]; // 3
    dbg!(day5::part2(
        &mut day5_sample_state,
        day5_sample_instructions
    ));
    let mut day5_state = vec![
        vec!['H', 'C', 'R'],                          // 1
        vec!['B', 'J', 'H', 'L', 'S', 'F'],           // 2
        vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],      // 3
        vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],      // 4
        vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'], // 5
        vec!['T', 'H', 'C', 'G'],                     // 6
        vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'], // 7
        vec!['R', 'J', 'Q', 'G', 'C'],                // 8
        vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'], // 9
    ];
    dbg!(day5::part2(&mut day5_state, &day5input));

    let day6sample = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
    let day6input = include_str!("../inputs/day6");
    dbg!(day6::part1(&day6sample));
    dbg!(day6::part1(&day6input));
    dbg!(day6::part2(&day6sample));
    dbg!(day6::part2(&day6input));
}
