# aoc-rs

My attempt at using Rust to solve Advent of Code puzzles.

## Usage

Run `cargo install aoc-rs-1npo` to download the crate.

Use the `solve` command to print the solution to the screen. For example, to get the solution for part 1 of day 1 in AoC 2024:

```
aoc-rs solve 2024 1 1
```
```
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::web] Got puzzle input from cached file
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Got puzzle input for year 2024 day 1
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Solution: "1651298"
```
Use the `submit` command to submit the solution as an answer to adventofcode.com. For example, to submit the solution for part 2 of day 1 in AoC 2024:

```
aoc-rs submit 2024 1 2
```
```
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::web] Got puzzle input from cached file
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Got puzzle input for year 2024 day 1
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Solution: "1651298"
[2024-12-03T01:29:14Z INFO  aoc_rs_1npo::puzzles] Puzzle solved! Great job!
```
## Acknowledgements

Thanks to Nir for his [`dayN.rs` template](https://github.com/quicknir/advent_rust/blob/main/advent_2023/src/bin/template.rs) and his [file caching code](https://github.com/quicknir/advent_rust/blob/e514ea70c66cb7359c00dbd8de0c1afe425d8aec/utils/src/file_utils.rs) :)