use itertools::Itertools;

type Compartments = (String, String);

pub fn get_priority(c: char) -> u32 {
    (c as u32) - if c.is_ascii_uppercase() { 64 - 26 } else { 96 }
}

#[aoc_generator(day3)]
pub fn parse_rucksack(input: &str) -> Vec<Compartments> {
    input
        .lines()
        .map(|l| {
            let mid = l.len() / 2;
            (l[..mid].to_owned(), l[mid..].to_owned())
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Compartments]) -> u32 {
    input
        .iter()
        .map(|(left, right)| {
            let dup = left.chars().find(|c| right.contains(*c)).unwrap();
            get_priority(dup)
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Compartments]) -> u32 {
    input
        .iter()
        .map(|(l, r)| format!("{}{}", l, r))
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| {
            let badge = a
                .chars()
                .filter(|ac| b.contains(*ac))
                .find(|abc| c.contains(*abc))
                .unwrap();
            get_priority(badge)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }
}
