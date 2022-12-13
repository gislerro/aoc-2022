use itertools::Itertools;

type Data = String;

#[aoc_generator(dayX)]
pub fn parse(input: &str) -> Data {
    todo!()
}

#[aoc(dayX, part1)]
pub fn solve_part1(input: &Data) -> usize {
    todo!()
}

#[aoc(dayX, part2)]
pub fn solve_part2(input: &Data) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 0);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 0);
    }
}
