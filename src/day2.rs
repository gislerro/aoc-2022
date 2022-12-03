use core::panic;




pub enum Action {
   ROCK,
   PAPER,
   SCISSOR
}

impl Action {

   fn outcome(&self, other: &Action) -> Outcome {
      use Action::{ROCK, PAPER, SCISSOR};
      match (self, other) {
         (ROCK, ROCK) => Outcome::DRAW,
         (ROCK, PAPER) => Outcome::LOSS,
         (ROCK, SCISSOR) => Outcome::WIN,

         (PAPER, ROCK) => Outcome::WIN,
         (PAPER, PAPER) => Outcome::DRAW,
         (PAPER, SCISSOR) => Outcome::LOSS,

         (SCISSOR, ROCK) => Outcome::LOSS,
         (SCISSOR, PAPER) => Outcome::WIN,
         (SCISSOR, SCISSOR) => Outcome::DRAW,
      }
   }

   fn score(&self, other: &Action) -> u32 {
      use Action::{ROCK, PAPER, SCISSOR};
      let own = match self {
         ROCK => 1,
         PAPER => 2,
         SCISSOR => 3,
      };
      use Outcome::{LOSS, DRAW, WIN};
      let outcome = match self.outcome(other) {
         LOSS => 0,
         DRAW => 3,
         WIN => 6
      };

      own + outcome
   }
}

enum Outcome {
   LOSS,
   DRAW,
   WIN
}

impl Outcome {
   pub fn score(&self, other: &Action) -> u32 {
      use Action::{ROCK, PAPER, SCISSOR};
      use Outcome::{LOSS, DRAW, WIN};
      match (self, other) {
         (LOSS, ROCK) => Action::SCISSOR,
         (LOSS, PAPER) => Action::ROCK,
         (LOSS, SCISSOR) => Action::PAPER,
         (DRAW, ROCK) => Action::ROCK,
         (DRAW, PAPER) => Action::PAPER,
         (DRAW, SCISSOR) => Action::SCISSOR,
         (WIN, ROCK) => Action::PAPER,
         (WIN, PAPER) => Action::SCISSOR,
         (WIN, SCISSOR) => Action::ROCK
      }.score(other)
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
            "A" => Action::ROCK,
            "B" => Action::PAPER,
            "C" => Action::SCISSOR,
            _ => panic!("unexpected character!")
         };
         ( opponent, parsed.next().unwrap().chars().next().unwrap() )
      }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> u32 {
   input
      .iter()
      .map(|(opponent, me)|
         match me {
         'X' => Action::ROCK,
         'Y' => Action::PAPER,
         'Z' => Action::SCISSOR,
         _ => panic!("unexpected character!")
         }.score(opponent)
      )
      .sum()
}


#[aoc(day2, part2)]
pub fn solve_part2(input: &[Round]) -> u32 {
   input
      .iter()
      .map(|(opponent, me)| {
         match me {
            'X' => Outcome::LOSS,
            'Y' => Outcome::DRAW,
            'Z' => Outcome::WIN,
            _ => panic!("unexpected character!")
         }.score(opponent)
      })
      .sum()
}