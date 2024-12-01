pub mod year2024;

use std::collections::HashMap;

/// This function lets us retrieve the function that solves the given part of a given
/// day's puzzle. The key is a tuple of u8 integers that represent the year, day, and
/// part. The value is the function that returns the solution for the given year, day,
/// and part.
///
/// For example, the value of key (2024, 1, 1) is `year2024::day1::part1()`, and the value
/// of key (2024, 1, 2) is `year2024::day1::part2()`.
pub fn get_puzzle_solution(
    year: u16,
    day: u8,
    part: u8,
    input: String,
) -> String {
    // A mapping of (day, part) tuples to functions in dayN.rs modules.
    //
    // When you add a new dayN.rs module, make sure to insert its part1 and part2
    // functions into this HashMap.
    let mut solutions: HashMap<(u16, u8, u8),
                                Box<dyn Fn(String) -> String>> = HashMap::new();

    solutions.insert((2024, 1, 1), Box::new(year2024::day1::part1));
    solutions.insert((2024, 1, 2), Box::new(year2024::day1::part2));

    solutions.get(&(year, day, part)).unwrap()(input)
}
