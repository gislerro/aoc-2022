use std::{fmt::Display, iter::repeat};

use crate::vec::Vec2;
use itertools::Itertools;
use num::integer::gcd;

#[derive(Copy, Clone)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{c}")
    }
}

impl From<Facing> for char {
    fn from(f: Facing) -> Self {
        match f {
            Facing::Right => '>',
            Facing::Down => 'v',
            Facing::Left => '<',
            Facing::Up => '^',
        }
    }
}

impl Facing {
    fn cw(&mut self) {
        *self = match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn ccw(&mut self) {
        *self = match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    fn dir(&self) -> Vec2 {
        // (0, 0) top left, y increases downwards
        match self {
            Self::Right => Vec2 { x: 1, y: 0 },
            Self::Down => Vec2 { x: 0, y: 1 },
            Self::Left => Vec2 { x: -1, y: 0 },
            Self::Up => Vec2 { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Go(u32),
    Cw,
    Ccw,
}

#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    square_size: usize,

    board: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl Board {
    // TODO: if let else?
    fn get(&self, position: &Vec2) -> Option<char> {
        if let Some(row) = self.board.get(position.y as usize) {
            if let Some(c) = row.get(position.x as usize) {
                return Some(*c);
            }
        }
        None
    }

    fn get_mut(&mut self, position: &Vec2) -> Option<&mut char> {
        if let Some(row) = self.board.get_mut(position.y as usize) {
            if let Some(c) = row.get_mut(position.x as usize) {
                return Some(c);
            }
        }
        None
    }

    fn start(&self) -> (Vec2, Facing) {
        let position = Vec2 {
            x: self.board[0]
                .iter()
                .enumerate()
                .find(|(_, &c)| c == '.')
                .unwrap()
                .0 as i32,
            y: 0,
        };
        let facing = Facing::Right;
        (position, facing)
    }

    fn wrap2d(&self, position: &Vec2, facing: &Facing) -> (Vec2, Facing) {
        let mut next = *position + facing.dir();
        next.x = next.x.rem_euclid(self.width as i32);
        next.y = next.y.rem_euclid(self.height as i32);
        while let Some(c) = self.get(&next) {
            if c != ' ' {
                break;
            }
            next += facing.dir();
            next.x = next.x.rem_euclid(self.width as i32);
            next.y = next.y.rem_euclid(self.height as i32);
        }
        (next, *facing)
    }

    fn square(&self, position: &Vec2) -> i32 {
        let a = position.x.rem_euclid(self.width as i32) / self.square_size as i32;
        let b = position.y.rem_euclid(self.height as i32) / self.square_size as i32;

        b * (self.width / self.square_size) as i32 + a
    }

    fn wrap3d(&self, position: &Vec2, facing: &Facing) -> (Vec2, Facing) {
        let mut next = *position + facing.dir();
        next.x = next.x.rem_euclid(self.width as i32);
        next.y = next.y.rem_euclid(self.height as i32);

        let from = self.square(position);
        let to = self.square(&next);
        if from == to {
            return self.wrap2d(position, facing);
        }

        if let Some(c) = self.get(&(*position + facing.dir())) {
            if c != ' ' {
                return self.wrap2d(position, facing);
            }
        }

        let k = self.square_size as i32;
        let x = position.x.rem_euclid(k);
        let y = position.y.rem_euclid(k);

        if self.square_size == 4 {
            match (from, to) {
                (6, 7) => (
                    Vec2 {
                        x: 3 * k + (k - 1) - y,
                        y: 2 * k,
                    },
                    Facing::Down,
                ),
                (10, 2) => (
                    Vec2 {
                        x: (k - 1) - x,
                        y: 2 * k - 1,
                    },
                    Facing::Up,
                ),
                (5, 1) => (Vec2 { x: 2 * k, y: x }, Facing::Right),
                _ => unimplemented!("implement transition from {from} to {to}!"),
            }
        } else {
            match (from, to) {
                (1, 10) => (Vec2 { x: 0, y: 3 * k + x }, Facing::Right),
                (9, 11) => (Vec2 { x: k + y, y: 0 }, Facing::Down),
                (1, 0) => (
                    Vec2 {
                        x: 0,
                        y: 2 * k + (k - 1) - y,
                    },
                    Facing::Right,
                ),
                (7, 10) => (
                    Vec2 {
                        x: k - 1,
                        y: 3 * k + x,
                    },
                    Facing::Left,
                ),
                (6, 8) => (
                    Vec2 {
                        x: k,
                        y: (k - 1) - y,
                    },
                    Facing::Right,
                ),
                (6, 3) => (Vec2 { x: k, y: k + x }, Facing::Right),
                (4, 3) => (Vec2 { x: y, y: 2 * k }, Facing::Down),
                (4, 5) => (
                    Vec2 {
                        x: 2 * k + y,
                        y: k - 1,
                    },
                    Facing::Up,
                ),
                (2, 5) => (
                    Vec2 {
                        x: 2 * k - 1,
                        y: k + x,
                    },
                    Facing::Left,
                ),
                (9, 0) => (Vec2 { x: 2 * k + x, y: 0 }, Facing::Down),
                (2, 11) => (Vec2 { x, y: 4 * k - 1 }, Facing::Up),
                (2, 0) => (
                    Vec2 {
                        x: 2 * k - 1,
                        y: 2 * k + (k - 1) - y,
                    },
                    Facing::Left,
                ),
                (7, 8) => (
                    Vec2 {
                        x: 3 * k - 1,
                        y: (k - 1) - y,
                    },
                    Facing::Left,
                ),
                (9, 10) => (
                    Vec2 {
                        x: k + y,
                        y: 3 * k - 1,
                    },
                    Facing::Up,
                ),
                _ => {
                    unimplemented!("implement transition from {from} to {to}, {facing}!");
                }
            }
        }
    }

    fn traverse(&mut self, dim: u32) -> (Vec2, Facing) {
        let (mut position, mut facing) = self.start();

        *self.get_mut(&position).unwrap() = facing.into();
        if cfg!(test) {
            println!("==== Init ====\n{self}");
        }
        for instr in self.instructions.clone().iter() {
            match instr {
                Instruction::Cw => facing.cw(),
                Instruction::Ccw => facing.ccw(),
                Instruction::Go(n) => {
                    for _ in 0..*n {
                        let (next, next_facing) = match dim {
                            2 => self.wrap2d(&position, &facing),
                            3 => self.wrap3d(&position, &facing),
                            _ => unimplemented!(),
                        };
                        let c = self
                            .get(&next)
                            .unwrap_or_else(|| panic!("next is an invalid position!: {next:?}"));
                        match c {
                            '#' => break,
                            ' ' => unreachable!("next is empty space!"),
                            _ => {
                                position = next;
                                facing = next_facing;
                                *self.get_mut(&position).unwrap() = facing.into();
                            }
                        }
                    }
                }
            }
            *self.get_mut(&position).unwrap() = facing.into();
            if cfg!(test) {
                println!("==== {instr:?} ====\n{self}");
            }
        }

        (position, facing)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?
        }
        Ok(())
    }
}

#[aoc_generator(day22)]
pub fn parse(input: &str) -> Board {
    let mut lines = input.lines();

    let board_lines = lines.take_while_ref(|l| !l.is_empty()).collect_vec();
    let n = board_lines.len();
    let m = board_lines.iter().map(|l| l.len()).max().unwrap();
    let k = gcd(n, m);

    let board = board_lines
        .iter()
        .map(|l| l.chars().chain(repeat(' ')).take(m).collect_vec())
        .collect_vec();

    let instructions = lines
        .nth(1)
        .unwrap()
        .split_inclusive(char::is_alphabetic)
        .flat_map(|s| {
            let mut num = s;
            let mut facing = "";
            if s.ends_with(char::is_alphabetic) {
                (num, facing) = s.split_at(s.len() - 1);
            }
            let facing = match facing {
                "R" => Some(Instruction::Cw),
                "L" => Some(Instruction::Ccw),
                _ => None,
            };

            [Some(Instruction::Go(num.parse().unwrap())), facing]
        })
        .flatten()
        .collect_vec();

    Board {
        height: n,
        width: m,
        square_size: k,
        board,
        instructions,
    }
}

#[aoc(day22, part1)]
pub fn solve_part1(board: &Board) -> i32 {
    let mut board = board.clone();
    let (p, f) = board.traverse(2);
    1000 * (p.y + 1) + 4 * (p.x + 1) + f as i32
}

#[aoc(day22, part2)]
pub fn solve_part2(board: &Board) -> i32 {
    let mut board = board.clone();
    let (p, f) = board.traverse(3);
    1000 * (p.y + 1) + 4 * (p.x + 1) + f as i32
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const PART2_SHAPE: &str = "     ..........
     ..........
     ..........
     ..........
     ..........
     .....
     .....
     .....
     .....
     .....
..........
..........
..........
..........
..........
.....
.....
.....
.....
.....

1L2R1R3R12R3R8L5R7R2R2L2L2L7L2R2L2L3L6R7L8L3L1L1L2L3R2R9L11R3L1L2";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 6032);
    }

    #[test]
    fn check_fold_part2() {
        let generated = super::parse(PART2_SHAPE);
        super::solve_part2(&generated);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 5031);
    }
}
