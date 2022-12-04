use itertools::Itertools;

use std::ops::RangeInclusive;

type Assignments = (RangeInclusive<u32>, RangeInclusive<u32>);


fn parse_range(range_str: &str) -> RangeInclusive<u32> {
   let mut parts = range_str.split('-');
   if let Ok(a) = parts.next().unwrap().parse::<u32>() {
      if let Ok(b) = parts.next().unwrap().parse::<u32>() {
         return a..=b
      }
   }
   unreachable!()
}

fn range_contained(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
   r1.start() <= r2.start() && r1.end() >= r2.end()
}

fn range_overlaps(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
   r1.start() <= r2.end() && r2.start() <= r1.end()
}

#[aoc_generator(day4)]
pub fn parse_assignments<'a>(input: &'a str) -> Vec<Assignments> {
   input
      .lines()
      .map(|l| {
         let mut split = l.split(',');
         (parse_range(split.next().unwrap()), parse_range(split.next().unwrap()))
      })
      .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Assignments]) -> u32 {
   input
      .iter()
      .filter(|(r1, r2)| {
         range_contained(r1, r2) || range_contained(r2, r1)
      })
      .count() as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Assignments]) -> u32 {
   input
      .iter()
      .filter(|(r1, r2)| {
         range_overlaps(r1, r2)
      })
      .count() as u32
}