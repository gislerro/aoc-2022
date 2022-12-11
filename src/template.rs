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

mod tests {

    #[test]
    fn check_part1() {
        const EXAMPLE1: &str = "1";
        let generated = super::parse(EXAMPLE1);
        assert_eq!(super::solve_part1(&generated), 0);
    }

    #[test]
    fn check_part2() {
        const EXAMPLE2: &str = "2";
        let generated = super::parse(EXAMPLE2);
        assert_eq!(super::solve_part2(&generated), 0);
    }
}
