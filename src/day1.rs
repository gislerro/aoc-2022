use itertools::Itertools;

type Calories = Vec<u32>;

#[aoc_generator(day1)]
pub fn parse_calories(input: &str) -> Vec<Calories> {
    let mut calories = Vec::new();
    calories.push(Vec::new());

    for line in input.lines() {
        if let Ok(n) = line.parse::<u32>() {
            if let Some(v) = calories.last_mut() {
                v.push(n);
            }
        } else {
            calories.push(Vec::new());
        }
    }
    calories
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Calories]) -> u32 {
    input
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap_or_default()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Calories]) -> u32 {
    input
        .iter()
        .map(|calories| calories.iter().sum())
        .sorted_by(|a, b: &u32| Ord::cmp(a, b).reverse())
        .take(3)
        .sum::<u32>()
}
