type DataStream = (usize, String);

#[aoc_generator(day6)]
pub fn parse(input: &str) -> DataStream {
    (input.len(), input.into())
}

fn find_marker(k: usize, (n, stream): &DataStream) -> usize {
    let mut seen: u32 = 0;
    let mut last_seen = vec![0; 26];
    let mut distinct: usize = 0;

    for i in 0..*n {
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
            if (seen >> m) & 1 == 1 && last_seen[m] == i - (k - 1) {
                seen &= !(1 << m);
                last_seen[m] = 0;
                distinct -= 1;
            }
        }
    }
    unreachable!();
}

#[aoc(day6, part1)]
pub fn solve_part1<'a>(stream: &DataStream) -> usize {
    find_marker(4, stream)
}

#[aoc(day6, part2)]
pub fn solve_part2(stream: &DataStream) -> usize {
    find_marker(14, stream)
}

mod tests {
    use super::*;

    const example1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn check_part2() {
        let generated = parse(example1);
        assert_eq!(solve_part2(&generated), 19);
    }
}
