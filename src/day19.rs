use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

type Mem = u8;

#[derive(Debug)]
pub struct Blueprint {
    ore: Mem,
    clay: Mem,
    obsidian: (Mem, Mem),
    geode: (Mem, Mem),
}

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ').filter_map(|x| x.parse::<Mem>().ok());
            Blueprint {
                ore: split.next().unwrap(),
                clay: split.next().unwrap(),
                obsidian: split.next_tuple().unwrap(),
                geode: split.next_tuple().unwrap(),
            }
        })
        .collect()
}

type Cache = HashSet<State>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Counts {
    ore: Mem,
    clay: Mem,
    obsidian: Mem,
    geode: Mem,
}

#[derive(Eq, PartialEq, Hash)]
struct State {
    minute: Mem,
    goods: Counts,
    robots: Counts,
}

fn gauss(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn solve<const MINUTES: u8>(
    blueprint: &Blueprint,
    state: State,
    max_robots: &Counts,
    memo: &mut Cache,
    best: &mut Mem,
) {
    if state.minute == MINUTES && state.goods.geode > *best {
        *best = state.goods.geode
    }

    let remaining = MINUTES as u32 - state.minute as u32;
    if state.goods.geode as u32 + state.robots.geode as u32 * remaining + gauss(remaining)
        <= *best as u32
    {
        return;
    }

    if memo.contains(&state) {
        return;
    }

    let next_goods = Counts {
        ore: state.goods.ore + state.robots.ore,
        clay: state.goods.clay + state.robots.clay,
        obsidian: state.goods.obsidian + state.robots.obsidian,
        geode: state.goods.geode + state.robots.geode,
    };

    // greedily build geode robot
    if state.goods.ore >= blueprint.geode.0
        && state.goods.obsidian >= blueprint.geode.1
        && state.robots.geode + 1 < max_robots.geode
    {
        solve::<MINUTES>(
            blueprint,
            State {
                minute: state.minute + 1,
                goods: Counts {
                    ore: next_goods.ore - blueprint.geode.0,
                    obsidian: next_goods.obsidian - blueprint.geode.1,
                    ..next_goods
                },
                robots: Counts {
                    geode: state.robots.geode + 1,
                    ..state.robots
                },
            },
            max_robots,
            memo,
            best,
        );
    } else {
        // just collect resources
        solve::<MINUTES>(
            blueprint,
            State {
                minute: state.minute + 1,
                goods: next_goods,
                robots: state.robots,
            },
            max_robots,
            memo,
            best,
        );

        // try building the others
        if state.goods.ore >= blueprint.ore && state.robots.ore + 1 < max_robots.ore {
            solve::<MINUTES>(
                blueprint,
                State {
                    minute: state.minute + 1,
                    goods: Counts {
                        ore: next_goods.ore - blueprint.ore,
                        ..next_goods
                    },
                    robots: Counts {
                        ore: state.robots.ore + 1,
                        ..state.robots
                    },
                },
                max_robots,
                memo,
                best,
            );
        }

        if state.goods.ore >= blueprint.clay && state.robots.clay + 1 < max_robots.clay {
            solve::<MINUTES>(
                blueprint,
                State {
                    minute: state.minute + 1,
                    goods: Counts {
                        ore: next_goods.ore - blueprint.clay,
                        ..next_goods
                    },
                    robots: Counts {
                        clay: state.robots.clay + 1,
                        ..state.robots
                    },
                },
                max_robots,
                memo,
                best,
            );
        }

        if state.goods.ore >= blueprint.obsidian.0
            && state.goods.clay >= blueprint.obsidian.1
            && state.robots.obsidian + 1 < max_robots.obsidian
        {
            solve::<MINUTES>(
                blueprint,
                State {
                    minute: state.minute + 1,
                    goods: Counts {
                        ore: next_goods.ore - blueprint.obsidian.0,
                        clay: next_goods.clay - blueprint.obsidian.1,
                        ..next_goods
                    },
                    robots: Counts {
                        obsidian: state.robots.obsidian + 1,
                        ..state.robots
                    },
                },
                max_robots,
                memo,
                best,
            )
        }
    }

    memo.insert(state);
}

fn max_geodes<const MINUTES: u8>(blueprint: &Blueprint) -> Mem {
    let mut cache = Cache::new();
    let state = State {
        minute: 0,
        goods: Counts {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Counts {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };
    let max_robots = Counts {
        ore: *[
            blueprint.ore,
            blueprint.clay,
            blueprint.obsidian.0,
            blueprint.geode.0,
        ]
        .iter()
        .max()
        .unwrap()
            + 1,
        clay: blueprint.obsidian.1 + 1,
        obsidian: blueprint.geode.1 + 1,
        geode: Mem::MAX,
    };

    let mut best = 0;
    solve::<MINUTES>(blueprint, state, &max_robots, &mut cache, &mut best);
    best
}

#[aoc(day19, part1)]
pub fn solve_part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .par_iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * max_geodes::<24>(b) as usize)
        .sum()
}

#[aoc(day19, part2)]
pub fn solve_part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .par_iter()
        .take(3)
        .map(|b| max_geodes::<32>(b) as usize)
        .product()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay.Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore.Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn check_part1() {
        let generated = super::parse(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 33);
    }
}
