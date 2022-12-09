use std::collections::VecDeque;

use itertools::Itertools;
use petgraph::{prelude::UnGraph, data::Build};


pub enum Node {
    Directory(String),
    File(String, usize)
}

type Graph = UnGraph::<Node, ()>;

#[aoc_generator(day7)]
pub fn parse_filesystem(input: &str) -> Graph {
    let mut graph = Graph::new_undirected();
    let mut cwd = VecDeque::<&str>::new();

    let root = graph.add_node(Node::Directory("root".to_owned()));
    let mut parent = root;

    for l in input.lines() {
        let split = l.split(' ');
        match split.next() {
            Some("$") => {
                match split.next() {
                    Some("cd") => {
                        match split.next() {
                            Some("..") => cwd.pop_front(),
                            Some(".") => cwd.clear(),
                            Some(cd) => {
                                cwd.push_front(cd)
                            }
                            _ => unreachable!()
                        }
                    }
                    Some("ls") => continue, // noop - or set the parent here?
                    _ => unreachable!()
                }
            },
            Some("dir") => {
                let dir = graph.add_node(Node::Directory(split.next().unwrap().into()));
                // TODO: set parent properly
                graph.add_edge(parent, dir, ());
            }
            Some(s) => {
                let size = s.parse::<usize>().unwrap();
                let file = graph.add_node(Node::File(split.next().unwrap().into(), size));
                // TODO: set parent properly
                graph.add_edge(parent, file, ());
            }
            _ => unreachable!()
        }
    }

    graph
}

fn parse_command(command: &str, graph: &mut Graph, cwd: &mut str) {

}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Graph) -> usize {
    todo!()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Graph) -> usize {
    todo!()
}

mod tests {

    #[test]
    fn check_part1() {
        const EXAMPLE1: &str = "1";
        let generated = super::parse(EXAMPLE1);
        assert_eq!(super::solve_part2(&generated), 0);
    }

    #[test]
    fn check_part2() {
        const EXAMPLE2: &str = "2";
        let generated = super::parse(EXAMPLE2);
        assert_eq!(super::solve_part2(&generated), 0);
    }
}
