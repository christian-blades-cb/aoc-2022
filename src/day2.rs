pub fn part1(xs: &str) -> usize {
    fn letter_to_shape(x: &str) -> Shape {
        match x {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }

    let mut score = 0;
    for ln in xs.lines() {
        if ln == "" {
            continue;
        }
        let mut parts = ln.split(' ').take(2);
        let attack = letter_to_shape(parts.next().unwrap());
        let response = letter_to_shape(parts.next().unwrap());
        let outcome = response.versus(&attack);
        score += response.score() + outcome.score();
    }
    score
}

pub fn part2(xs: &str) -> usize {
    let mut score = 0;
    for ln in xs.lines() {
        if ln == "" {
            continue;
        }
        let mut parts = ln.split(' ').take(2);
        let attack = Shape::from_str(parts.next().unwrap());
        let outcome = Outcome::from_str(parts.next().unwrap());
        let response = Shape::from_outcome(&attack, &outcome);

        score += response.score() + outcome.score()
    }
    score
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn from_str(x: &str) -> Self {
        match x {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn from_str(x: &str) -> Self {
        match x {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => unreachable!(),
        }
    }

    fn versus(self, x: &Self) -> Outcome {
        match (self, x) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Lose,
            (Self::Rock, Self::Scissors) => Outcome::Win,

            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissors) => Outcome::Lose,

            (Self::Scissors, Self::Rock) => Outcome::Lose,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Scissors, Self::Scissors) => Outcome::Draw,
        }
    }

    fn score(self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn from_outcome(attack: &Self, outcome: &Outcome) -> Self {
        match (attack, outcome) {
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,

            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,

            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        }
    }
}
