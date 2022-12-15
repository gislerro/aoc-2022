use std::{cmp::max, cmp::min, collections::HashSet};

use itertools::Itertools;

type Coord = (i32, i32);
type Rocks = HashSet<Coord>;

#[aoc_generator(day14)]
pub fn parse(input: &str) -> (Rocks, i32) {
    let mut rocks = HashSet::<Coord>::new();

    let abyss = input
        .replace(" -> ", ",")
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|n| n.parse::<i32>().ok())
                .tuples()
                .tuple_windows()
                .map(|((a, b), (c, d))| {
                    for x in min(a, c)..=max(a, c) {
                        for y in min(b, d)..=max(b, d) {
                            rocks.insert((x, y));
                        }
                    }
                    max(b, d)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    (rocks, abyss)
}

fn simulate_grain<F>(
    sim_while: F,
    floor: Option<i32>,
    rocks: &HashSet<Coord>,
    sand: &HashSet<Coord>,
) -> Option<Coord>
where
    F: Fn(Coord) -> bool,
{
    let mut grain: Coord = (500, 0);

    while sim_while(grain) {
        if let Some(next) = [(0, 1), (-1, 1), (1, 1)].iter().find_map(|(dx, dy)| {
            let next = (grain.0 + dx, grain.1 + dy);
            if rocks.contains(&next) || sand.contains(&next) {
                return None;
            }
            if let Some(y) = floor {
                if next.1 == y {
                    return None;
                }
            }
            Some(next)
        }) {
            grain = next;
        } else {
            return Some(grain);
        }
    }

    None
}

#[aoc(day14, part1)]
pub fn solve_part1((rocks, abyss): &(Rocks, i32)) -> usize {
    let mut sand = HashSet::<Coord>::new();

    while let Some(coord) = simulate_grain(|grain| grain.1 < *abyss, None, rocks, &sand) {
        sand.insert(coord);
    }

    sand.len()
}

#[aoc(day14, part2)]
pub fn solve_part2((rocks, abyss): &(Rocks, i32)) -> usize {
    let mut sand = HashSet::<Coord>::new();

    while let Some(coord) =
        simulate_grain(|_| !sand.contains(&(500, 0)), Some(abyss + 2), rocks, &sand)
    {
        sand.insert(coord);
    }

    sand.len()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 24);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 93);
    }
}
