use std::collections::VecDeque;

type Coord = (usize, usize);

pub struct HeightMap {
    heights: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
}

impl HeightMap {
    fn width(&self) -> usize {
        self.heights[0].len()
    }

    fn height(&self) -> usize {
        self.heights.len()
    }

    fn get(&self, (i, j): Coord) -> Option<&u8> {
        self.heights.get(i).and_then(|row| row.get(j))
    }

    fn neighbors(&self, (i, j): Coord) -> impl Iterator<Item = Coord> + '_ {
        let h = self.heights[i][j];

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |(di, dj)| {
                let ii = (i as i32) + di;
                let jj = (j as i32) + dj;

                if let Some(nh) = self.get((ii as usize, jj as usize)) {
                    // reversed conditions - since we find the path in reverse
                    if *nh == h - 1 || *nh >= h {
                        Some((ii as usize, jj as usize))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }
}

fn find_mutate<F>(parsed: &mut [Vec<u8>], fun: F) -> Option<(usize, usize)>
where
    F: Fn(&mut u8) -> bool,
{
    parsed.iter_mut().enumerate().find_map(|(i, row)| {
        row.iter_mut()
            .enumerate()
            .find_map(|(j, c)| if fun(c) { Some(j) } else { None })
            .map(|j| (i, j))
    })
}

#[aoc_generator(day12)]
pub fn parse_heightmap(input: &str) -> HeightMap {
    let mut heights = Vec::from_iter(input.lines().map(|l| l.as_bytes().to_vec()));

    let start = find_mutate(&mut heights, |c| {
        if *c == b'S' {
            *c = b'a';
            return true;
        }
        false
    })
    .expect("couldnt find start");

    let end = find_mutate(&mut heights, |c| {
        if *c == b'E' {
            *c = b'z';
            return true;
        }
        false
    })
    .expect("couldnt find end");

    HeightMap {
        heights,
        start,
        end,
    }
}

#[derive(Copy, Clone)]
struct Path {
    cost: usize,
    coord: Coord,
}

fn distance<F>(map: &HeightMap, termination: F) -> usize
where
    F: Fn(Coord) -> bool,
{
    let mut dist = vec![vec![std::usize::MAX; map.width()]; map.height()];
    // uniform distance - no prioqueue needed
    let mut queue = VecDeque::<Path>::new();

    // compute all distances from the end (thus solving part 1 & 2 together)
    dist[map.end.0][map.end.1] = 0;
    queue.push_back(Path {
        cost: 0,
        coord: map.end,
    });

    // BFS
    while let Some(Path { cost, coord }) = queue.pop_front() {
        if termination(coord) {
            return cost;
        }

        if cost > dist[coord.0][coord.1] {
            continue;
        }

        for ncoord in map.neighbors(coord) {
            let next = Path {
                cost: cost + 1,
                coord: ncoord,
            };

            if next.cost < dist[next.coord.0][next.coord.1] {
                queue.push_back(next);
                dist[next.coord.0][next.coord.1] = next.cost;
            }
        }
    }

    unreachable!("couldnt find termination criteria!");
}

#[aoc(day12, part1)]
pub fn solve_part1(map: &HeightMap) -> usize {
    distance(map, |coord| coord == map.start)
}

#[aoc(day12, part2)]
pub fn solve_part2(map: &HeightMap) -> usize {
    distance(map, |coord| *map.get(coord).unwrap() == b'a')
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn check_part1() {
        let generated = super::parse_heightmap(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 31);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse_heightmap(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 29);
    }
}
