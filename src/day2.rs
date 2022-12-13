use core::panic;

pub enum Action {
    Rock,
    Paper,
    Scissor,
}

impl Action {
    fn outcome(&self, other: &Action) -> Outcome {
        use Action::{Paper, Rock, Scissor};
        match (self, other) {
            (Rock, Rock) => Outcome::Draw,
            (Rock, Paper) => Outcome::Loss,
            (Rock, Scissor) => Outcome::Win,

            (Paper, Rock) => Outcome::Win,
            (Paper, Paper) => Outcome::Draw,
            (Paper, Scissor) => Outcome::Loss,

            (Scissor, Rock) => Outcome::Loss,
            (Scissor, Paper) => Outcome::Win,
            (Scissor, Scissor) => Outcome::Draw,
        }
    }

    fn score(&self, other: &Action) -> u32 {
        use Action::{Paper, Rock, Scissor};
        let own = match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        use Outcome::{Draw, Loss, Win};
        let outcome = match self.outcome(other) {
            Loss => 0,
            Draw => 3,
            Win => 6,
        };

        own + outcome
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(&self, other: &Action) -> u32 {
        use Action::{Paper, Rock, Scissor};
        use Outcome::{Draw, Loss, Win};
        match (self, other) {
            (Loss, Rock) => Action::Scissor,
            (Loss, Paper) => Action::Rock,
            (Loss, Scissor) => Action::Paper,
            (Draw, Rock) => Action::Rock,
            (Draw, Paper) => Action::Paper,
            (Draw, Scissor) => Action::Scissor,
            (Win, Rock) => Action::Paper,
            (Win, Paper) => Action::Scissor,
            (Win, Scissor) => Action::Rock,
        }
        .score(other)
    }
}

pub type Round = (Action, char);

#[aoc_generator(day2)]
pub fn parse_strategy(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|l| {
            let mut parsed = l.trim().split(' ');
            let opponent = match parsed.next().unwrap() {
                "A" => Action::Rock,
                "B" => Action::Paper,
                "C" => Action::Scissor,
                _ => panic!("unexpected character!"),
            };
            (opponent, parsed.next().unwrap().chars().next().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> u32 {
    input
        .iter()
        .map(|(opponent, me)| {
            match me {
                'X' => Action::Rock,
                'Y' => Action::Paper,
                'Z' => Action::Scissor,
                _ => panic!("unexpected character!"),
            }
            .score(opponent)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Round]) -> u32 {
    input
        .iter()
        .map(|(opponent, me)| {
            match me {
                'X' => Outcome::Loss,
                'Y' => Outcome::Draw,
                'Z' => Outcome::Win,
                _ => panic!("unexpected character!"),
            }
            .score(opponent)
        })
        .sum()
}
