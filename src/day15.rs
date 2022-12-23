use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;

pub struct Diamond {
    center: (i32, i32),
    radius: u32,
}

impl Diamond {
    fn segment_by_y(&self, y: i32) -> Option<Range<i32>> {
        let dy = self.center.1.abs_diff(y);
        if dy > self.radius {
            None
        } else {
            let x = self.center.0;
            let dx = self.radius - dy;
            let x1 = x - dx as i32;
            let x2 = x + dx as i32;
            Some(x1..x2)
        }
    }
}

#[aoc_generator(day15)]
pub fn parse(input: &str) -> Vec<Diamond> {
    input
        .replace("x=", "")
        .replace("y=", "")
        .replace([':', ','], "")
        .lines()
        .map(|l| {
            l.split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .tuples()
                .map(|(x1, y1, x2, y2)| {
                    let p = (x1, y1);
                    let d = x1.abs_diff(x2) + y1.abs_diff(y2);
                    Diamond {
                        center: p,
                        radius: d,
                    }
                })
                .next()
                .expect("error parsing a line!")
        })
        .collect_vec()
}

fn find_occupied(y: i32, diamonds: &[Diamond]) -> Vec<Range<i32>> {
    let mut segments = diamonds
        .iter()
        .filter_map(|d| d.segment_by_y(y))
        .sorted_by(|a, b| Ord::cmp(&a.start, &b.start));

    let mut merged = vec![segments.next().unwrap()];
    for segment in segments {
        let top = merged.last().unwrap();
        if top.end < segment.start {
            merged.push(segment);
        } else if top.end < segment.end {
            let top = merged.last_mut().unwrap();
            top.end = segment.end;
        }
    }

    merged
}

#[aoc(day15, part1)]
pub fn solve_part1(diamonds: &[Diamond]) -> usize {
    let occupied = find_occupied(2_000_000, diamonds);

    let mut result = 0;
    for m in occupied {
        result += m.end - m.start + 1;
    }

    result as usize - 1
}

#[aoc(day15, part2)]
pub fn solve_part2(diamonds: &[Diamond]) -> i64 {
    (0..4_000_000)
        .into_par_iter()
        .find_map_any(|y| {
            let occupied = find_occupied(y, diamonds);
            if occupied.len() == 2 {
                let x = occupied[0].end + 1;
                Some((x as i64) * 4_000_000 + y as i64)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 26);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 56000011);
    }
}
