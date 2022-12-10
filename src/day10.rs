pub enum Instruction {
    Noop,
    Addx(i32),
}

#[aoc_generator(day10)]
pub fn parse_instruction(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            match split.next() {
                Some("noop") => Instruction::Noop,
                Some("addx") => Instruction::Addx(
                    split
                        .next()
                        .map(|s| s.parse::<i32>().expect("error parsing addx"))
                        .unwrap(),
                ),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> i32 {
    let targets = [20, 60, 100, 140, 180, 220];
    let mut k = 0;

    let mut register = 1;
    let mut cycle = 0;

    let mut sum_strength = 0;

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                cycle += 1;
                if cycle == targets[k] {
                    sum_strength += cycle * register;
                    k += 1;
                    if k == targets.len() {
                        break;
                    }
                }
            }
            Instruction::Addx(x) => {
                cycle += 2;

                let mut changed = false;
                if cycle - targets[k] == 1 {
                    sum_strength += (cycle - 1) * register;
                    changed = true;
                }
                if cycle == targets[k] {
                    sum_strength += cycle * register;
                    changed = true;
                }
                if changed {
                    k += 1;
                    if k == targets.len() {
                        break;
                    }
                }

                register += x;
            }
        }
    }

    sum_strength
}

fn draw_pixel(cycle: usize, sprite: i32, screen: &mut [Vec<char>]) {
    let width = screen[0].len();

    let line = cycle / width;
    let target = cycle % width;

    (-1..=1).for_each(|mut x: i32| {
        x += sprite;
        if x >= 0 && x < width as i32 && target as i32 == x {
            screen[line][x as usize] = '#'
        }
    })
}

#[aoc(day10, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> String {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    let mut screen = vec![vec!['.'; WIDTH]; HEIGHT];

    let mut sprite = 1;
    let mut cycle = 0;

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                draw_pixel(cycle, sprite, &mut screen);
                cycle += 1;
            }
            Instruction::Addx(x) => {
                draw_pixel(cycle, sprite, &mut screen);
                draw_pixel(cycle + 1, sprite, &mut screen);

                cycle += 2;
                sprite += x;
            }
        }
    }

    screen
        .iter()
        .enumerate()
        .map(|(i, scanline)| {
            let mut s = scanline.iter().collect::<String>();

            if i == 0 {
                s.insert(0, '\n');
            }

            if i != HEIGHT - 1 {
                s.push('\n');
            }

            s
        })
        .collect::<String>()
}

mod tests {

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn check_part1() {
        let generated = super::parse_instruction(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 13140);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse_instruction(EXAMPLE);
        assert_eq!(
            super::solve_part2(&generated),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
