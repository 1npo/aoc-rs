# aoc-rs
This is my attempt at using Rust to build an Advent of Code puzzle solver. It's also my attempt to learn Rust!

## Install
```console
cargo install aoc-rs-1npo
```

## Usage
```console
aoc-rs <COMMAND> <YEAR> <DAY> <PART>
```

The first three arguments are required, `PART` defaults to 1 if absent.

There are 2 commands: `solve` and `submit`.

Use the `solve` command to print the solution to the screen.

For example:

```console
aoc-rs solve 2024 1 1
```
```console
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::web] Got puzzle input from cached file
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Got puzzle input for year 2024 day 1
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Solution for part 1 = "1651298"
```

Use the `submit` command to submit the solution as an answer to adventofcode.com.

For example:

```console
aoc-rs submit 2024 1 2
```
```console
[2024-12-03T01:35:46Z INFO  aoc_rs_1npo::web] Got puzzle input from cached file
[2024-12-03T01:35:46Z INFO  aoc_rs_1npo::puzzles] Got puzzle input for year 2024 day 1
[2024-12-03T01:35:46Z INFO  aoc_rs_1npo::puzzles] Solution for part 2 = "21306195"
[2024-12-03T01:35:46Z INFO  aoc_rs_1npo::puzzles] Puzzle solved! Great job!
```

## Developer Guide

### Adding Solutions for New Days
Follow these steps to add solutions for a new day.

Replace `DAY`, `PART`, `YYYY`, and `N` with the appropriate values.

1. Create `src/puzzles/yearYYYY/dayN.rs`
2. Add `pub mod dayN;` to `src/puzzles/yearYYYY/mod.rs`
3. Add `solutions.insert((2024, DAY, PART), Box::new(yearYYYY::dayN::partN));` to `get_puzzle_solution()` in `src/puzzles/mod.rs` 
4. Implement the input parsing for the day's puzzle in `yearYYYY::dayN::parse()`
5. Implement the solutions for parts 1 and 2 in `yearYYYY::dayN::part1()` and `yearYYYY::dayN::part2()` respectively

### Adding a New Year
1. Create `src/puzzles/yearYYYY`
2. Create `src/puzzles/yearYYYY/mod.rs`
3. Follow the steps above to create `dayN.rs` files for the year

## Acknowledgements
Thanks to Nir for his [`dayN.rs` template](https://github.com/quicknir/advent_rust/blob/main/advent_2023/src/bin/template.rs) and his [file caching code](https://github.com/quicknir/advent_rust/blob/e514ea70c66cb7359c00dbd8de0c1afe425d8aec/utils/src/file_utils.rs) :)