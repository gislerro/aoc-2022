AOC 2022 Solutions written in Rust using [Cargo AOC Helper](https://github.com/gobanos/cargo-aoc)

## Solving a puzzle
### Writing a generator (parser)
A single annotated method which can return a custom type:
```
#[aoc_generator(dayX)]
pub fn parse(input: &str) -> Vec<T> {
   ...
}
```
can either take in a `&str` or `&[u8]`

### Writing a solver
Takes in a slice of the parsed input and solves the (sub) task:
```
#[aoc(dayX, partX)]
pub fn solve_partX(input: &[T]) -> u32 {
   ...
}
```

## Running a solution
`cargo aoc` to run the latest implemented day (and downloading the input file if needed)

`cargo aoc -d {day} -p {part}` for running a specific part

## Benchmarking
`cargo aoc bench -o` to open the benchmark result directly in the browser
