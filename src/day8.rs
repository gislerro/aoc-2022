use std::iter;

use itertools::Itertools;

type TreeGrid = Vec<Vec<u32>>;

#[aoc_generator(day8)]
pub fn parse(input: &str) -> TreeGrid {
    input.lines()
        .map(|l| {
            l.chars().filter_map(|c| c.to_digit(10)).collect_vec()
        })
        .collect()
}

fn occlusion<R>(height: u32, range: R, trees: &TreeGrid) -> (bool, usize) 
where
    R: Iterator<Item = (usize, usize)>
{
    let mut distance = 0;
    for (i, j) in range {
        distance += 1;
        if trees[i][j] >= height {
            return (true, distance)
        }
    } 
    
    (false, distance)
}

// Naive, brute force solution
fn solve((i, j): (usize, usize), trees: &TreeGrid) -> (bool, usize) {

    let width = trees[i].len();
    let height = trees.len();

    let tree_height = trees[i][j];
    

    let mut visible = false;
    let mut scenic = 1;

    // left
    let (occluded, distance) = occlusion(tree_height, iter::repeat(i).zip((0..j).rev()), trees);
    if !occluded {
        visible = true;
    }
    scenic *= distance;

    // right
    let (occluded, distance) = occlusion(tree_height, iter::repeat(i).zip(j+1..width), trees);
    if !occluded {
        visible = true;
    }
    scenic *= distance;

    // top
    let (occluded, distance) = occlusion(tree_height, ((0..i).rev()).zip(iter::repeat(j)), trees);
    if !occluded {
        visible = true;
    }
    scenic *= distance;

    // bottom
    let (occluded, distance) = occlusion(tree_height, (i+1..height).zip(iter::repeat(j)), trees);
    if !occluded {
        visible = true;
    }
    scenic *= distance;

    (visible, scenic)
}

#[aoc(day8, part1)]
pub fn solve_part1(trees: &TreeGrid) -> usize {
    trees.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| solve((i, j), trees).0)
                .filter(|v| *v)
                .count()
        })
        .sum::<usize>()
}

#[aoc(day8, part2)]
pub fn solve_part2(trees: &TreeGrid) -> usize {
    trees.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| solve((i, j), trees).1)
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default()
}

mod tests {

    const EXAMPLE: &str = "30373
    25512
    65332
    33549
    35390";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 21);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 8);
    }
}
