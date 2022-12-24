use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::vec::Vec2;
use itertools::Itertools;
use num::integer::lcm;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for Vec2 {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => Vec2 { x: 0, y: -1 },
            Direction::Down => Vec2 { x: 0, y: 1 },
            Direction::Left => Vec2 { x: -1, y: 0 },
            Direction::Right => Vec2 { x: 1, y: 0 },
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "^",
                Self::Down => "v",
                Self::Left => "<",
                Self::Right => ">",
            }
        )
    }
}

type Blizzard = HashMap<Vec2, Vec<Direction>>;

pub struct Basin {
    start: Vec2,
    end: Vec2,

    width: usize,
    height: usize,

    blizzard: Blizzard,
}

impl Basin {
    fn wall(&self, p: &Vec2) -> bool {
        if *p == self.start || *p == self.end {
            return false;
        }
        if p.x <= 0 || p.x >= self.width as i32 - 1 {
            return true;
        }
        if p.y <= 0 || p.y >= self.height as i32 - 1 {
            return true;
        }
        false
    }

    fn blizzard_at(&self, k: usize) -> Blizzard {
        self.blizzard
            .iter()
            .flat_map(move |(&p, ds)| {
                ds.iter().map(move |&d| {
                    let dp: Vec2 = d.into();
                    let mut p = p - Vec2 { x: 1, y: 1 };
                    p = p + dp * k as i32;
                    p.x = p.x.rem_euclid(self.width as i32 - 2) + 1;
                    p.y = p.y.rem_euclid(self.height as i32 - 2) + 1;
                    (p, d)
                })
            })
            .fold(HashMap::<Vec2, Vec<Direction>>::new(), |mut map, (p, d)| {
                if let Some(ds) = map.get_mut(&p) {
                    ds.push(d);
                } else {
                    map.insert(p, vec![d]);
                }
                map
            })
    }
}

#[aoc_generator(day24)]
pub fn parse(input: &str) -> Basin {
    let lines = input.lines().collect_vec();
    let width = lines[0].len();
    let height = lines.len();

    let start = Vec2 {
        x: lines[0]
            .chars()
            .enumerate()
            .find_map(|(x, c)| if c == '.' { Some(x) } else { None })
            .unwrap() as i32,
        y: 0,
    };

    let end = Vec2 {
        x: lines[height - 1]
            .chars()
            .enumerate()
            .find_map(|(x, c)| if c == '.' { Some(x) } else { None })
            .unwrap() as i32,
        y: height as i32 - 1,
    };

    let blizzard = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                let p = Vec2 {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '^' => Some((p, vec![Direction::Up])),
                    'v' => Some((p, vec![Direction::Down])),
                    '>' => Some((p, vec![Direction::Right])),
                    '<' => Some((p, vec![Direction::Left])),
                    _ => None,
                }
            })
        })
        .collect::<HashMap<_, _>>();

    Basin {
        start,
        end,
        width,
        height,
        blizzard,
    }
}

fn solve(
    start: &Vec2,
    end: &Vec2,
    basin: &Basin,
    blizzard: usize,
    blizzards: &[Blizzard],
) -> usize {
    let k = blizzards.len();

    let mut positions = HashSet::new();
    positions.insert(*start);

    for i in blizzard.. {
        let mut next_positions = HashSet::new();
        for p in positions.iter() {
            if p == end {
                return i - blizzard;
            }

            let next_blizzard = &blizzards[(i + 1) % k];

            // adjacent moves
            for adj in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next = *p + adj.into();
                if !basin.wall(&next) && !next_blizzard.contains_key(&next) {
                    next_positions.insert(next);
                }
            }

            // wait
            if !next_blizzard.contains_key(p) {
                next_positions.insert(*p);
            }
        }
        positions = next_positions;
    }
    unreachable!()
}

#[aoc(day24, part1)]
pub fn solve_part1(basin: &Basin) -> usize {
    let num_blizzards = lcm(basin.width - 2, basin.height - 2);
    let blizzards: Vec<Blizzard> = (0..num_blizzards)
        .into_par_iter()
        .map(|k| basin.blizzard_at(k))
        .collect();
    solve(&basin.start, &basin.end, basin, 0, &blizzards)
}

#[aoc(day24, part2)]
pub fn solve_part2(basin: &Basin) -> usize {
    let num_blizzards = lcm(basin.width - 2, basin.height - 2);
    let blizzards: Vec<Blizzard> = (0..num_blizzards)
        .into_par_iter()
        .map(|k| basin.blizzard_at(k))
        .collect();
    let a = solve(&basin.start, &basin.end, basin, 0, &blizzards);
    let b = solve(&basin.end, &basin.start, basin, a, &blizzards);
    let c = solve(&basin.start, &basin.end, basin, a + b, &blizzards);

    println!("{a}, {b}, {c}");

    a + b + c
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 18);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 54);
    }
}
