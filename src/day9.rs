use std::{collections::HashSet, iter};

type Move = (char, i32);
type Position = (i32, i32);

struct Rope<const SIZE: usize> {
    knots: [Position; SIZE],
    visited: HashSet<Position>,
}

impl<const SIZE: usize> Rope<SIZE> {
    fn new() -> Rope<SIZE> {
        let mut rope = Rope {
            knots: [(0, 0); SIZE],
            visited: HashSet::new(),
        };
        rope.visited.insert((0, 0));
        rope
    }

    fn step(&mut self, (dx, dy): Position) {
        let mut head = &mut self.knots[0];

        head.0 += dx;
        head.1 += dy;

        for i in 1..SIZE {
            if !self.touching(i) {
                self.repair(i)
            }
        }

        self.visited.insert(self.knots[SIZE - 1]);
    }

    fn touching(&self, i: usize) -> bool {
        let before = self.knots[i - 1];
        let current = self.knots[i];

        let dx = (before.0 - current.0).abs();
        let dy = (before.1 - current.1).abs();

        dx < 2 && dy < 2
    }

    fn repair(&mut self, i: usize) {
        let before = self.knots[i - 1];
        let mut current = &mut self.knots[i];

        let dx = before.0 - current.0;
        let dy = before.1 - current.1;

        current.0 += dx.signum();
        current.1 += dy.signum();
    }
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .filter_map(|l| {
            let mut split = l.split(' ');

            let direction = split.next().and_then(|s| s.chars().next());
            let amount = split.next().and_then(|n| n.parse::<i32>().ok());

            if let (Some(direction), Some(amount)) = (direction, amount) {
                Some((direction, amount))
            } else {
                None
            }
        })
        .collect()
}

fn make_steps<const SIZE: usize>(moves: &[Move]) -> usize {
    let mut rope = Rope::<SIZE>::new();

    for (direction, amount) in moves {
        for step in iter::repeat(match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        })
        .take(*amount as usize)
        {
            rope.step(step);
        }
    }

    rope.visited.len()
}

#[aoc(day9, part1)]
pub fn solve_part1(moves: &[Move]) -> usize {
    make_steps::<2>(moves)
}

#[aoc(day9, part2)]
pub fn solve_part2(moves: &[Move]) -> usize {
    make_steps::<10>(moves)
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_part1() {
        const EXAMPLE: &str = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 13);
    }

    #[test]
    fn check_part2() {
        const EXAMPLE: &str = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";

        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 36);
    }
}
