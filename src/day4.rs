use std::ops::RangeInclusive;

use itertools::Itertools;

type Assignments = (RangeInclusive<u32>, RangeInclusive<u32>);

fn parse_range(range_str: &str) -> RangeInclusive<u32> {
    range_str
        .split('-')
        .take(2)
        .filter_map(|s| s.parse::<u32>().ok())
        .next_tuple()
        .map(|(a, b)| a..=b)
        .unwrap()
}

fn range_contained(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    r1.start() <= r2.start() && r1.end() >= r2.end()
}

fn range_overlaps(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    r1.start() <= r2.end() && r2.start() <= r1.end()
}

#[aoc_generator(day4)]
pub fn parse_assignments(input: &str) -> Vec<Assignments> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .take(2)
                .map(|r_str| parse_range(r_str))
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Assignments]) -> u32 {
    input
        .iter()
        .filter(|(r1, r2)| range_contained(r1, r2) || range_contained(r2, r1))
        .count() as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Assignments]) -> u32 {
    input
        .iter()
        .filter(|(r1, r2)| range_overlaps(r1, r2))
        .count() as u32
}
