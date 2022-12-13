use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};

enum Order {
    Right,
    Wrong,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl Packet {
    fn parse(chars: &mut impl Iterator<Item = char>) -> Option<Self> {
        match chars.next() {
            Some('[') => {
                let mut packets = Vec::new();
                while let Some(packet) = Packet::parse(chars) {
                    packets.push(packet)
                }
                Some(Packet::List(packets))
            }
            Some(']') => None,
            Some('a') => Some(Packet::Integer(10)), // kek
            Some(n) => Some(Packet::Integer(
                n.to_digit(10).expect("couldn't parse integer!"),
            )),
            _ => unreachable!(),
        }
    }

    fn order(&self, other: &Packet) -> Option<Order> {
        match (self, other) {
            (Packet::Integer(x), Packet::Integer(y)) => match x.cmp(y) {
                Ordering::Less => Some(Order::Right),
                Ordering::Greater => Some(Order::Wrong),
                Ordering::Equal => None,
            },
            (a, b) => {
                let xs = match a {
                    Packet::Integer(x) => vec![Packet::Integer(*x)],
                    Packet::List(xs) => xs.clone(),
                };

                let ys = match b {
                    Packet::Integer(y) => vec![Packet::Integer(*y)],
                    Packet::List(ys) => ys.clone(),
                };

                xs.iter()
                    .zip_longest(ys.iter())
                    .find_map(|pair| match pair {
                        EitherOrBoth::Both(x, y) => x.order(y),
                        EitherOrBoth::Left(_) => Some(Order::Wrong),
                        EitherOrBoth::Right(_) => Some(Order::Right),
                    })
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.order(other) {
            Some(Order::Right) => Ordering::Less,
            Some(Order::Wrong) => Ordering::Greater,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|xs| Packet::parse(&mut xs.replace("10", "a").replace(',', "").chars()))
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(packets: &[Packet]) -> usize {
    packets
        .iter()
        .tuples()
        .enumerate()
        .map(|(i, (a, b))| match a.order(b) {
            Some(Order::Right) => i + 1,
            Some(Order::Wrong) => 0,
            None => unreachable!(),
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(packets: &[Packet]) -> usize {
    let divider = parse("[[2]]\n[[6]]");

    let a = &divider[0];
    let b = &divider[1];

    divider
        .iter()
        .chain(packets.iter())
        .sorted()
        .enumerate()
        .filter_map(|(i, x)| if x == a || x == b { Some(i + 1) } else { None })
        .product()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 13);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 140);
    }
}
