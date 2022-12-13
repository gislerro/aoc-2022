pub struct File {
    //name: String,
    size: usize,
}

pub struct Directory {
    name: String,

    index: usize,
    parent: Option<usize>,

    directories: Vec<usize>,
    files: Vec<File>,
}

#[aoc_generator(day7)]
pub fn parse_filesystem(input: &str) -> Vec<Directory> {
    let mut dirs = vec![Directory {
        name: "/".to_owned(),

        index: 0,
        parent: None,

        directories: vec![],
        files: vec![],
    }];

    // root has index 0
    let mut cwd = 0;

    for l in input.lines() {
        let mut split = l.split(' ');
        match split.next() {
            Some("$") => {
                match split.next() {
                    Some("cd") => match split.next() {
                        Some("..") => {
                            cwd = dirs[cwd]
                                .parent
                                .expect("cannot cd .. out of root directory!")
                        }
                        Some("/") => cwd = 0,
                        Some(cd) => {
                            cwd = dirs[cwd]
                                .directories
                                .iter()
                                .find_map(|d| {
                                    if dirs[*d].name == cd {
                                        Some(dirs[*d].index)
                                    } else {
                                        None
                                    }
                                })
                                .expect("couldn't find cd dir!");
                        }
                        _ => unreachable!(),
                    },
                    Some("ls") => continue, // noop
                    _ => unreachable!(),
                }
            }
            Some("dir") => {
                let name = split.next().unwrap().to_owned();

                let k = dirs.len();

                dirs.push(Directory {
                    name,
                    index: k,
                    parent: Some(cwd),
                    directories: vec![],
                    files: vec![],
                });

                dirs[cwd].directories.push(k);
            }
            Some(s) => {
                let size = s.parse::<usize>().unwrap();
                // actually unused
                //let name = split.next().unwrap().to_owned();

                dirs[cwd].files.push(File { size });
            }
            _ => unreachable!(),
        }
    }

    dirs
}

fn compute_directory_size(cwd: usize, dirs: &[Directory], sizes: &mut [usize]) -> usize {
    let fs = dirs[cwd].files.iter().map(|f| f.size).sum::<usize>();
    let ds = dirs[cwd]
        .directories
        .iter()
        .map(|d| compute_directory_size(*d, dirs, sizes))
        .sum::<usize>();

    sizes[cwd] = fs + ds;
    sizes[cwd]
}

#[aoc(day7, part1)]
pub fn solve_part1(directories: &[Directory]) -> usize {
    let mut sizes = vec![0; directories.len()];

    compute_directory_size(0, directories, &mut sizes);

    sizes.iter().filter(|&ds| *ds <= 100000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(directories: &[Directory]) -> usize {
    let mut sizes = vec![0; directories.len()];

    compute_directory_size(0, directories, &mut sizes);

    let used = sizes[0];
    let unused = 70000000 - used;
    let target = 30000000 - unused;

    *sizes.iter().filter(|&ds| ds >= &target).min().unwrap()
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn check_part1() {
        let generated = super::parse_filesystem(EXAMPLE);
        assert_eq!(super::solve_part1(&generated), 95437);
    }

    #[test]
    fn check_part2() {
        let generated = super::parse_filesystem(EXAMPLE);
        assert_eq!(super::solve_part2(&generated), 24933642);
    }
}
