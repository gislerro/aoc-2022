use std::collections::{HashMap, HashSet};

type Coord = (i64, i64);

// for convenience all length 5
// left edge at 0, 0 and aligned with bottom where possible
// only special case: plus there we have to add one to the height
const HORIZONTAL: [Coord; 5] = [(0, 0), (1, 0), (2, 0), (3, 0), (0, 0)];
const PLUS: [Coord; 5] = [(0, 0), (1, 0), (2, 0), (1, -1), (1, 1)];
const CORNER: [Coord; 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const VERTICAL: [Coord; 5] = [(0, 0), (0, 1), (0, 2), (0, 3), (0, 0)];
const SQUARE: [Coord; 5] = [(0, 0), (0, 1), (1, 0), (1, 1), (0, 0)];

fn get_rock(idx: usize) -> &'static [Coord; 5] {
    match idx {
        0 => &HORIZONTAL,
        1 => &PLUS,
        2 => &CORNER,
        3 => &VERTICAL,
        4 => &SQUARE,
        _ => unreachable!(),
    }
}

type Fallen = HashSet<(i64, i64)>;

#[aoc_generator(day17)]
pub fn parse(input: &str) -> String {
    input.to_owned()
}

fn affected(position: &Coord, rock: &[Coord; 5], fallen: &mut Fallen, dx: i64) -> bool {
    for part in rock.iter().take(5) {
        let x = part.0 + position.0 + dx;
        let y = part.1 + position.1;

        if x < 0 || x == 7 || fallen.contains(&(x, y)) {
            return false;
        }
    }
    true
}

fn rest(position: &Coord, rock: &[Coord; 5], fallen: &mut Fallen) -> bool {
    for part in rock.iter().take(5) {
        let x = part.0 + position.0;
        let y = part.1 + position.1;

        if fallen.contains(&(x, y - 1)) || y == 0 {
            return true;
        }
    }
    false
}

fn fall(
    rock_idx: usize,
    wind_idx: &mut usize,
    winds: &[u8],
    height: &mut i64,
    fallen: &mut Fallen,
) {
    let rock = get_rock(rock_idx);

    let mut position = (2, *height + 3);
    if rock_idx == 1 {
        position.1 += 1;
    }

    loop {
        let wind = winds[*wind_idx];
        *wind_idx = (*wind_idx + 1usize) % winds.len();

        position.0 += match wind {
            b'>' => {
                if affected(&position, rock, fallen, 1) {
                    1
                } else {
                    0
                }
            }
            b'<' => {
                if affected(&position, rock, fallen, -1) {
                    -1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };

        if rest(&position, rock, fallen) {
            for i in 0..5 {
                let rock = get_rock(rock_idx);
                let part = rock[i];
                let x = position.0 + part.0;
                let y = position.1 + part.1;

                if y + 1 > *height {
                    *height = y + 1
                };

                fallen.insert((x, y));
            }
            break;
        }

        position.1 -= 1;
    }
}

fn _print(height: i64, fallen: &Fallen) {
    for y in (0..height).rev() {
        print!("|");
        for x in 0..7 {
            if fallen.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("|");
    }
    println!("+-------+\n\n")
}

#[aoc(day17, part1)]
pub fn solve_part1(winds: &String) -> i64 {
    let mut fallen = HashSet::new();
    let winds = winds.as_bytes();

    let mut rock_idx = 0;
    let mut wind_idx = 0;

    let mut height = 0;

    for _ in 0..2022 {
        fall(rock_idx, &mut wind_idx, winds, &mut height, &mut fallen);
        rock_idx = (rock_idx + 1) % 5;
        //print(height, &fallen);
    }

    height
}

fn relief(height: i64, fallen: &Fallen) -> [u32; 50] {
    let mut r = [0; 50];
    for h in 1..=50 {
        let y = height - h;
        for x in 0..7 {
            if fallen.contains(&(x, y)) {
                r[h as usize - 1] |= 1 << x;
            }
        }
    }
    r
}

#[aoc(day17, part2)]
pub fn solve_part2(winds: &String) -> i64 {
    let mut fallen = HashSet::new();

    let target = 1_000_000_000_000;
    //let target = 2022;

    let mut cycles = HashMap::<[u32; 50], (i64, i64)>::new();

    let winds = winds.as_bytes();

    let mut rock_idx = 0;
    let mut wind_idx = 0;

    let mut height = 0;

    let mut rounds_remaining = 0;
    let mut round_delta = 0;
    let mut height_delta = 0;

    for i in 1i64.. {
        fall(rock_idx, &mut wind_idx, winds, &mut height, &mut fallen);
        let relief = relief(height, &fallen);

        rock_idx = (rock_idx + 1) % 5;

        if let Some((k, h)) = cycles.get(&relief) {
            println!("detected cycle between {k} to {i}, heights: {h} -> {height}");
            rounds_remaining = target - i;
            round_delta = i - k;
            height_delta = height - h;
            break;
        } else {
            cycles.insert(relief, (i, height));
        }
    }

    let num_cycles = rounds_remaining / round_delta;
    let sim = rounds_remaining % round_delta;

    // simulate the remaining rounds to reach the target
    for _ in 0..sim {
        fall(rock_idx, &mut wind_idx, winds, &mut height, &mut fallen);
        rock_idx = (rock_idx + 1) % 5;
    }

    // add the height from the cycles to the result
    height += num_cycles * height_delta;
    height
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 3068);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 1514285714288);
    }
}
