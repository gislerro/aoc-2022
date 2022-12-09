#[aoc_generator(day6)]
pub fn parse(input: &str) -> String {
    input.into()
}

fn find_marker(k: usize, stream: &String) -> usize {
    let mut seen: u32 = 0;
    let mut last_seen = vec![0; 26];
    let mut distinct: usize = 0;

    for i in 0..stream.len() {
        // input is ASCII so can index by bytes
        let m = (stream.as_bytes()[i] - b'a') as usize;
        if (seen >> m) & 1 == 0 {
            seen |= 1 << m;
            distinct += 1;
        }
        last_seen[m] = i;

        if distinct == k {
            return i + 1;
        }

        if i >= k - 1 {
            let m = (stream.as_bytes()[i - (k - 1)] - b'a') as usize;
            if last_seen[m] == i - (k - 1) {
                seen &= !(1 << m);
                distinct -= 1;
            }
        }
    }
    unreachable!();
}

#[aoc(day6, part1)]
pub fn solve_part1(stream: &String) -> usize {
    find_marker(4, stream)
}

#[aoc(day6, part2)]
pub fn solve_part2(stream: &String) -> usize {
    find_marker(14, stream)
}

mod tests {

    #[test]
    fn check_part2() {
        const EXAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let generated = super::parse(EXAMPLE1);
        assert_eq!(super::solve_part2(&generated), 19);
    }
}
