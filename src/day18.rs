use std::{
    cmp::Ordering,
    collections::HashSet,
    ops::{Add, Sub},
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

const ADJACENT: [Cube; 6] = [
    Cube { x: -1, y: 0, z: 0 },
    Cube { x: 1, y: 0, z: 0 },
    Cube { x: 0, y: -1, z: 0 },
    Cube { x: 0, y: 1, z: 0 },
    Cube { x: 0, y: 0, z: -1 },
    Cube { x: 0, y: 0, z: 1 },
];

impl Cube {
    fn sides(&self) -> impl Iterator<Item = Cube> + '_ {
        ADJACENT.iter().map(|adj| *self + *adj)
    }

    // 12 sides in total
    fn reachable_sides<'a>(
        &'a self,
        outside: &'a Cube,
    ) -> impl Iterator<Item = (Cube, Cube, Option<Cube>)> + '_ {
        let delta = *outside - *self;

        // consider the sides adjacent to the current cube (4 sides)
        // !! this move is only valid when adj + delta is not a cube aswell !!
        let opposite = *self - *outside;
        let one = self.sides().filter_map(move |adj| {
            if adj != *outside && adj != opposite {
                Some((*self, adj, Some(adj + delta)))
            } else {
                None
            }
        });

        // consider the sides in the same plane as the given side (4 sides)
        let two = self.sides().filter_map(move |adj| {
            let outoutside = adj + delta;
            if adj != *outside && outoutside != *self {
                Some((adj, adj + delta, None))
            } else {
                None
            }
        });

        // consider the sides adjacent to the outside cube (4 sides)
        let outoutside = *outside + delta;
        let three = outside.sides().filter_map(move |adj| {
            if adj != outoutside && adj != *self {
                Some((adj, *outside, None))
            } else {
                None
            }
        });

        one.chain(two).chain(three)
    }
}

impl Add for Cube {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cube {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Cube {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Cube {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Equal => self.z.cmp(&other.z),
                o => o,
            },
            o => o,
        }
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day18)]
pub fn parse(input: &str) -> HashSet<Cube> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|x| x.parse::<i32>().ok())
                .tuples()
                .next()
                .map(|(x, y, z)| Cube { x, y, z })
                .unwrap()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|a| {
            ADJACENT
                .iter()
                .filter(|b| !cubes.contains(&(*a + **b)))
                .count()
        })
        .sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(cubes: &HashSet<Cube>) -> usize {
    let mut visited = HashSet::<(Cube, Cube)>::new();

    let mut stack = Vec::<(Cube, Cube)>::new();
    let min = cubes.iter().min().unwrap();
    stack.push((*min, Cube { x: -1, y: 0, z: 0 } + *min));

    while let Some((a, b)) = stack.pop() {
        visited.insert((a, b));
        for (c, d, e) in a.reachable_sides(&b) {
            if cubes.contains(&c) && !cubes.contains(&d) && !visited.contains(&(c, d)) {
                if let Some(e) = e {
                    if cubes.contains(&e) {
                        continue;
                    }
                }
                stack.push((c, d))
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::Cube;

    const EXAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 64);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 58);
    }

    #[test]
    fn check_reachable() {
        let a = Cube { x: 0, y: 0, z: 0 };
        let b = Cube { x: 0, y: 1, z: 0 };
        a.reachable_sides(&b).for_each(|c| println!("{:?}", c));
    }
}
