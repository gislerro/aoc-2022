use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};

#[derive(Clone)]
pub enum Operation {
    Add(u32),
    Mult(u32),
    Square,
}

#[derive(Clone)]
pub struct Test {
    divisor: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone)]
pub struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    test: Test,
}

fn parse_collect<C, T>(input_str: &str) -> C
where
    T: FromStr,
    C: FromIterator<T>,
{
    input_str
        .replace(',', "")
        .split(' ')
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<C>()
}

fn parse_first<T>(input_str: &str) -> Option<T>
where
    T: FromStr,
{
    input_str.split(' ').find_map(|s| s.parse::<T>().ok())
}

#[aoc_generator(day11)]
pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .replace("  ", "")
        .lines()
        .filter(|l| !l.is_empty())
        .tuples()
        .map(|(_, items, operation, divisor, if_true, if_false)| {
            let mut op_split = operation.split(' ').skip(4);
            let op = match op_split.next() {
                Some("*") => match op_split.next().and_then(|x| x.parse::<u32>().ok()) {
                    Some(n) => Operation::Mult(n),
                    None => Operation::Square,
                },
                Some("+") => Operation::Add(
                    op_split
                        .next()
                        .and_then(|n| n.parse::<u32>().ok())
                        .expect("no addend found"),
                ),
                _ => unreachable!(),
            };

            Monkey {
                items: parse_collect(items),
                op,
                test: Test {
                    divisor: parse_first(divisor).expect("no divisor found"),
                    if_true: parse_first(if_true).expect("no if true found"),
                    if_false: parse_first(if_false).expect("no if false found"),
                },
            }
        })
        .collect()
}

fn round(monkeys: &mut [Monkey], modulo: Option<usize>, inspection: &mut [usize]) {
    let k = monkeys.len();
    for i in 0..k {
        while !monkeys[i].items.is_empty() {
            inspection[i] += 1;

            let monkey = &mut monkeys[i];
            let mut worry = monkey.items.pop_front().unwrap();

            match monkey.op {
                Operation::Add(n) => worry += n as usize,
                Operation::Mult(n) => worry *= n as usize,
                Operation::Square => worry *= worry,
            };

            match modulo {
                Some(m) => worry %= m,
                None => worry /= 3,
            }

            let next_monkey = if worry % monkey.test.divisor == 0 {
                &mut monkeys[monkey.test.if_true]
            } else {
                &mut monkeys[monkey.test.if_false]
            };

            next_monkey.items.push_back(worry);
        }
    }
}

fn solve<const N: usize>(monkeys: &mut [Monkey], modulo: Option<usize>) -> usize {
    let k = monkeys.len();
    let mut inspection: Vec<usize> = vec![0; k];

    for _ in 0..N {
        round(monkeys, modulo, &mut inspection);
    }

    inspection.sort();
    inspection[k - 2] * inspection[k - 1]
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> usize {
    // AOC runner doesn't let me take in &mut [Monkey]
    let mut monkeys: Vec<Monkey> = input.to_vec();
    solve::<20>(&mut monkeys, None)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> usize {
    // AOC runner doesn't let me take in &mut [Monkey]
    let mut monkeys: Vec<Monkey> = input.to_vec();
    let modulo = monkeys.iter().map(|m| m.test.divisor).product();
    solve::<10_000>(&mut monkeys, Some(modulo))
}

mod tests {

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn check_part1() {
        let generated = super::parse_monkeys(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 10605);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse_monkeys(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 2713310158);
    }
}
