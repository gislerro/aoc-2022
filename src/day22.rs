use crate::vec::Vec2;
use itertools::Itertools;

#[derive(Copy, Clone)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
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
    fn cw(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn ccw(&self) -> Self {
        match self {
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

#[derive(Debug)]
pub enum Instruction {
    Go(u32),
    Cw,
    Ccw,
}

type Board = Vec<Vec<char>>;
type Parsed = (Board, Vec<Instruction>);

#[aoc_generator(day22)]
pub fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();

    let mut board = lines
        .take_while_ref(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let m = board.iter().map(|row| row.len()).max().unwrap();
    board
        .iter_mut()
        .for_each(|row| row.append(&mut vec![' '; m - row.len()]));

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

    (board, instructions)
}

enum Wrap {
    Left,
    Right,
    Top,
    Bottom,
    None,
}

fn wrap2d(p: &Vec2, d: &Vec2, board: &Board) -> Vec2 {
    let next = *p + *d;
    let n = board.len();
    let m = board[0].len();

    let wrap = if next.x < 0 {
        Wrap::Right
    } else if next.x >= m as i32 {
        Wrap::Left
    } else if next.y < 0 {
        Wrap::Bottom
    } else if next.y >= n as i32 {
        Wrap::Top
    } else {
        match board[next.y as usize][next.x as usize] {
            '#' => Wrap::None,
            '.' | '>' | '<' | 'v' | '^' => Wrap::None,
            ' ' => match (d.x, d.y) {
                (1, 0) => Wrap::Left,
                (-1, 0) => Wrap::Right,
                (0, 1) => Wrap::Top,
                (0, -1) => Wrap::Bottom,
                (_, _) => unreachable!(),
            },
            _ => unreachable!(),
        }
    };

    match wrap {
        Wrap::None => next,
        Wrap::Left => board[p.y as usize]
            .iter()
            .enumerate()
            .find_map(|(i, c)| {
                if *c != ' ' {
                    Some(Vec2 {
                        x: i as i32,
                        y: p.y,
                    })
                } else {
                    None
                }
            })
            .unwrap(),
        Wrap::Right => board[p.y as usize]
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, c)| {
                if *c != ' ' {
                    Some(Vec2 {
                        x: i as i32,
                        y: p.y,
                    })
                } else {
                    None
                }
            })
            .unwrap(),
        Wrap::Top => board
            .iter()
            .enumerate()
            .find_map(|(i, cs)| {
                if cs[p.x as usize] != ' ' {
                    Some(Vec2 {
                        x: p.x,
                        y: i as i32,
                    })
                } else {
                    None
                }
            })
            .unwrap(),
        Wrap::Bottom => board
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, cs)| {
                if cs[p.x as usize] != ' ' {
                    Some(Vec2 {
                        x: p.x,
                        y: i as i32,
                    })
                } else {
                    None
                }
            })
            .unwrap(),
    }
}

fn move2d(k: u32, p: &mut Vec2, facing: &Facing, board: &mut Board) {
    let d = facing.dir();
    for _ in 0..k {
        let n = wrap2d(p, &d, board);

        match board[n.y as usize][n.x as usize] {
            '#' => {
                board[p.y as usize][p.x as usize] = char::from(*facing);
                return;
            }
            '.' | '>' | '<' | 'v' | '^' => {
                board[p.y as usize][p.x as usize] = char::from(*facing);
                *p = n;
            }
            ' ' => unreachable!("wrap isn't working properly!"),
            _ => unreachable!(),
        }
    }
}

#[aoc(day22, part1)]
pub fn solve_part1((board, instructions): &Parsed) -> i32 {
    let mut p = Vec2 {
        x: board[0]
            .iter()
            .enumerate()
            .find_map(|(i, c)| if *c == '.' { Some(i as i32) } else { None })
            .unwrap(),
        y: 0,
    };

    let mut facing = Facing::Right;
    let mut board = board.to_owned();

    for instr in instructions {
        match instr {
            Instruction::Cw => facing = facing.cw(),
            Instruction::Ccw => facing = facing.ccw(),
            Instruction::Go(k) => {
                move2d(*k, &mut p, &facing, &mut board);
            }
        }
    }

    1000 * (p.y + 1) + 4 * (p.x + 1) + facing as i32
}

#[aoc(day22, part2)]
pub fn solve_part2((board, instructions): &Parsed) -> i32 {
    let mut p = Vec2 {
        x: board[0]
            .iter()
            .enumerate()
            .find_map(|(i, c)| if *c == '.' { Some(i as i32) } else { None })
            .unwrap(),
        y: 0,
    };

    let mut facing = Facing::Right;
    let mut board = board.to_owned();

    for instr in instructions {
        match instr {
            Instruction::Cw => facing = facing.cw(),
            Instruction::Ccw => facing = facing.ccw(),
            Instruction::Go(k) => {
                move2d(*k, &mut p, &facing, &mut board);
            }
        }
    }

    1000 * (p.y + 1) + 4 * (p.x + 1) + facing as i32
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

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 6032);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 0);
    }
}
