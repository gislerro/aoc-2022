use scanf::sscanf;
use std::collections::HashMap;

pub struct Node {
    flow: u32,
    nonzero_idx: usize,
    neighbors: Vec<usize>,
}

type Parsed = (Vec<Node>, usize);

#[aoc_generator(day16)]
pub fn parse(input: &str) -> (Vec<Node>, usize) {
    let mut map = HashMap::<String, usize>::new();
    let mut cave = Vec::<Node>::new();
    let mut start = 0;

    for l in input
        .replace("valves", "valve")
        .replace("tunnels", "tunnel")
        .replace("leads", "lead")
        .lines()
    {
        let mut x = String::new();
        let mut f = 0;
        let mut e = String::new();

        if sscanf!(
            l,
            "Valve {string} has flow rate={u32}; tunnel lead to valve {string}",
            x,
            f,
            e
        )
        .is_ok()
        {
            let from = if let Some(idx) = map.get(&x) {
                if let Some(n) = cave.get_mut(*idx) {
                    n.flow = f;
                }
                *idx
            } else {
                let idx = cave.len();
                cave.push(Node {
                    flow: f,
                    nonzero_idx: 0,
                    neighbors: Vec::new(),
                });

                map.insert(x.clone(), idx);
                idx
            };

            if x == "AA" {
                start = from;
            }

            for to in e.replace(',', "").split(' ') {
                let to = if let Some(idx) = map.get(to) {
                    *idx
                } else {
                    let idx = cave.len();
                    cave.push(Node {
                        flow: 0,
                        nonzero_idx: 0,
                        neighbors: Vec::new(),
                    });

                    map.insert(to.to_owned(), idx);
                    idx
                };
                if let Some(n) = cave.get_mut(from) {
                    n.neighbors.push(to);
                }
            }
        } else {
            unreachable!("{:?}", l)
        }
    }

    let mut k = 0;
    for room in cave.iter_mut() {
        if room.flow > 0 {
            room.nonzero_idx = k;
            k += 1
        }
    }

    (cave, start)
}

fn maxflow(
    minute: usize,
    location: usize,
    opened: usize,
    dp: &mut [Vec<Vec<i32>>],
    cave: &[Node],
) -> i32 {
    let minutes = dp.len();

    if minute == minutes {
        return 0;
    }

    if dp[minute][location][opened] != -1 {
        return dp[minute][location][opened];
    }

    let room = &cave[location];

    // consider moving to neighbor
    let mut max = 0;
    for to in room.neighbors.iter() {
        max = std::cmp::max(maxflow(minute + 1, *to, opened, dp, cave), max);
    }

    // if valve not yet opened and has positive flow rate open it
    if opened & (1 << room.nonzero_idx) == 0 && room.flow > 0 {
        let open = opened | 1 << room.nonzero_idx;
        let added_flow = (minutes as i32 - (minute as i32 + 1)) * room.flow as i32;
        max = std::cmp::max(
            added_flow + maxflow(minute + 1, location, open, dp, cave),
            max,
        );
    }

    dp[minute][location][opened] = max;
    max
}

#[aoc(day16, part1)]
pub fn solve_part1((cave, start): &Parsed) -> i32 {
    let n = cave.len();

    let k = cave.iter().filter(|n| n.flow > 0).count();
    let mut dp = vec![vec![vec![-1i32; 1 << k]; n]; 30];

    maxflow(0, *start, 0, &mut dp, cave)
}

#[aoc(day16, part2)]
pub fn solve_part2((cave, start): &Parsed) -> i32 {
    let n = cave.len();
    let k = cave.iter().filter(|n| n.flow > 0).count();

    let mut dp = vec![vec![vec![-1i32; 1 << k]; n]; 26];

    let mut max = 0;

    // me and elephant have to open disjoint valves so find the maximum by considering all possible disjoint pairs
    // due to symmetry can skip one half
    for opened in 0..1 << (k - 1) {
        let eopened = !opened & ((1 << k) - 1);

        let me = maxflow(0, *start, eopened, &mut dp, cave);
        let elephant = maxflow(0, *start, opened, &mut dp, cave);

        max = std::cmp::max(me + elephant, max)
    }
    max
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 1651);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 1707);
    }
}
