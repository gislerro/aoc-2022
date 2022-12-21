use itertools::Itertools;

#[aoc_generator(day20)]
pub fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .collect()
}

fn mix_pos(i: usize, original: &[i64], indices: &mut Vec<usize>, key: i64) {
    let n = original.len() as i64;
    let amount = original[i];
    let from = indices
        .iter()
        .enumerate()
        .find_map(|(k, &ii)| if ii == i { Some(k) } else { None })
        .unwrap();

    let mut to = from as i64 + amount * key;

    // wrap around
    to = to.rem_euclid(n - 1);

    let ii = indices.remove(from);
    indices.insert(to as usize, ii);
}

fn mix<const ROUNDS: usize>(original: &[i64], key: i64) -> Vec<usize> {
    let n = original.len();
    let mut indices = (0..n).collect_vec();

    for _ in 0..ROUNDS {
        for i in 0..n {
            mix_pos(i, original, &mut indices, key);
        }
    }
    indices
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {
    let mixed = mix::<1>(input, 1);

    let zero = mixed
        .iter()
        .enumerate()
        .find_map(|(i, ii)| if input[*ii] == 0 { Some(i) } else { None })
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|&i| {
            let ii = (zero + i) % input.len();
            input[mixed[ii]]
        })
        .sum()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let key = 811589153;
    let mixed = mix::<10>(input, key);

    let zero = mixed
        .iter()
        .enumerate()
        .find_map(|(i, ii)| if input[*ii] == 0 { Some(i) } else { None })
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|&i| {
            let ii = (zero + i) % input.len();
            input[mixed[ii]] * key
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 3);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 1623178306);
    }
}
