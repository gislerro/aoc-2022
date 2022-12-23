use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

use crate::vec::Vec2;

const N: Vec2 = Vec2 { x: 0, y: -1 };
const NE: Vec2 = Vec2 { x: 1, y: -1 };
const E: Vec2 = Vec2 { x: 1, y: 0 };
const SE: Vec2 = Vec2 { x: 1, y: 1 };
const S: Vec2 = Vec2 { x: 0, y: 1 };
const SW: Vec2 = Vec2 { x: -1, y: 1 };
const W: Vec2 = Vec2 { x: -1, y: 0 };
const NW: Vec2 = Vec2 { x: -1, y: -1 };

const ADJ: [Vec2; 8] = [N, NE, E, SE, S, SW, W, NW];

const NORTH: [Vec2; 3] = [N, NE, NW];
const SOUTH: [Vec2; 3] = [S, SE, SW];
const WEST: [Vec2; 3] = [W, NW, SW];
const EAST: [Vec2; 3] = [E, NE, SE];

pub struct Board {
    elves: HashSet<Vec2>,
}

impl FromIterator<Vec2> for Board {
    fn from_iter<T: IntoIterator<Item = Vec2>>(iter: T) -> Self {
        Board {
            elves: iter.into_iter().collect(),
        }
    }
}

impl ToOwned for Board {
    type Owned = Self;
    fn to_owned(&self) -> Self::Owned {
        Board {
            elves: self.elves.to_owned(),
        }
    }
}

impl Board {
    fn extent(&self) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
        let x = self.elves.iter().minmax_by_key(|v| v.x);
        let y = self.elves.iter().minmax_by_key(|v| v.y);

        match (x, y) {
            (MinMax(xmin, xmax), MinMax(ymin, ymax)) => (xmin.x..=xmax.x, ymin.y..=ymax.y),
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Update {
    old: Vec2,
    new: Vec2,
}

impl Hash for Update {
    // only hash the new position - so Iterator::counts() can be used below
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.new.hash(state)
    }
}

impl PartialEq for Update {
    fn eq(&self, other: &Self) -> bool {
        self.new == other.new
    }
}
impl Eq for Update {}

#[aoc_generator(day23)]
pub fn parse(input: &str) -> Board {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some(Vec2 {
                        x: x as i32,
                        y: y as i32,
                    }),
                    _ => None,
                })
                .collect_vec()
        })
        .collect()
}

fn diffuse<F>(board: &mut Board, termination: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let mut cyclic = [NORTH, SOUTH, WEST, EAST, NORTH, SOUTH, WEST]
        .array_windows::<4>()
        .cycle();

    for i in 1.. {
        let order = cyclic.next().unwrap();
        let num_updates = board
            .elves
            .iter()
            .filter(|&p| ADJ.iter().any(|&dp| board.elves.contains(&(*p + dp))))
            .filter_map(|p| {
                order.iter().find_map(|check| {
                    if check.iter().all(|&dp| !board.elves.contains(&(*p + dp))) {
                        Some(Update {
                            old: *p,
                            new: *p + check[0],
                        })
                    } else {
                        None
                    }
                })
            })
            .counts()
            .iter()
            .filter_map(
                |(&update, &count)| {
                    if count == 1 {
                        Some(update)
                    } else {
                        None
                    }
                },
            )
            .map(|Update { old, new }| {
                board.elves.remove(&old);
                board.elves.insert(new);
            })
            .count();

        if termination(i, num_updates) {
            return i;
        }
    }
    unreachable!()
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Board) -> usize {
    let mut board = input.to_owned();
    diffuse(&mut board, |round, _| round == 10);
    let (xs, ys) = board.extent();
    xs.count() * ys.count() - board.elves.len()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Board) -> usize {
    let mut board = input.to_owned();
    diffuse(&mut board, |_, num_updates| num_updates == 0)
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 110);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 20);
    }
}
