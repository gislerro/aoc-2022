use std::collections::VecDeque;

use itertools::Itertools;

type Stack = VecDeque<char>;
type Move = (usize, usize, usize);

type Crates = (Vec<Stack>, Vec<Move>);

fn parse_supply_stacks<'a, L>(stacks: L) -> Vec<Stack> 
   where L: Iterator<Item = &'a str>
{
   let mut parsed = vec![VecDeque::new(); 9];

   stacks
      .for_each(|l| {
         l.char_indices()
            .for_each(|(i, c)| {
               if i % 4 == 1 && c != ' ' { // all crates satisfy this
                  parsed[i / 4].push_back(c)
               }
            })
      });

   parsed
}

fn parse_move(move_str: &str) -> Move {
   move_str
      .split(' ')
      .filter_map(|s| s.parse::<usize>().ok())
      .next_tuple()
      .unwrap()
}

#[aoc_generator(day5)]
pub fn parse_cargo(input: &str) -> Crates {

   let mut lines = input.lines();

   let stack_strs = lines.take_while_ref(|&l| !l.is_empty());

   let stacks = parse_supply_stacks(stack_strs);

   // skip over empty line
   lines.next();

   let moves = lines
      .map(parse_move)
      .collect();

   (stacks, moves)
}

#[aoc(day5, part1)]
pub fn solve_part1((stacks, moves): &Crates) -> String {
   let mut stacks = stacks.clone();

   for (n, a, b) in moves {
      for _ in 0..*n {
         let moved = stacks[a-1].pop_front().unwrap();
         stacks[b-1].push_front(moved);
      }
   }

   stacks.iter()
      .map(|s| s.front().unwrap())
      .collect()
}

#[aoc(day5, part2)]
pub fn solve_part2((stacks, moves): &Crates) -> String {
   let mut stacks = stacks.clone();

   for (n, a, b) in moves {
      let moved = stacks[a-1].drain(0..*n).collect_vec();
      moved.iter().rev().for_each(|c| stacks[b-1].push_front(*c));
   }

   stacks.iter()
      .map(|s| s.front().unwrap())
      .collect()
}