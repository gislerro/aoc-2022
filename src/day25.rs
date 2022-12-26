use itertools::Itertools;
use take_until::*;

fn from_snafu(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let n = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };
            n * 5i64.pow(i as u32)
        })
        .sum()
}

fn to_snafu(n: i64) -> String {
    // numerical range a k-digit snafu number can represent
    let ranges = (0..)
        .map(|k| {
            let max: i64 = (0..=k).rev().map(|i| 2 * 5i64.pow(i)).sum();
            (-max)..=max
        })
        .take_until(|range| range.contains(&n))
        .collect_vec();

    let mut snafu = String::new();
    let mut rem = n;
    for k in (0..ranges.len()).rev() {
        let pow = 5i64.pow(k as u32);

        let (d, r) = match k {
            0 => (rem, 0),
            _ => [-2, -1, 1, 2]
                .iter()
                .find_map(|&d| {
                    let v = d * pow;
                    let r = rem - v;
                    if ranges[k - 1].contains(&r) {
                        Some((d, r))
                    } else {
                        None
                    }
                })
                .unwrap_or((0, rem)),
        };
        rem = r;

        snafu += match d {
            -2 => "=",
            -1 => "-",
            0 => "0",
            1 => "1",
            2 => "2",
            _ => unreachable!(),
        };
    }

    snafu
}

#[aoc_generator(day25)]
pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(snafu: &[String]) -> String {
    let base10 = snafu.iter().map(|s| from_snafu(s)).sum();
    println!("{base10}");
    to_snafu(base10)
}

#[aoc(day25, part2)]
pub fn solve_part2(_input: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), "2=-1=0");
    }

    #[test]
    fn check_snafu() {
        let generated = super::parse(EXAMPLE);
        let decode_encode = generated
            .iter()
            .map(|s| super::from_snafu(s))
            .map(|n| super::to_snafu(n))
            .collect_vec();
        assert_eq!(generated, decode_encode);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 0);
    }
}
