pub mod year2024;

use std::collections::HashMap;
use log::info;

use crate::{Cli, Method, get_input, post_puzzle_answer};

/// 1. Get the puzzle input for the given year and day (eg 2024, 1)
/// 2. Run the function that solves the given part of the puzzle (ie 1 or 2)
/// 3. Return the puzzle results using the given method:
///     - Solve: Print the puzzle solution
///     - Submit: Submit the puzzle solution to adventofcode.com and print a message
///         indicating whether or not the solution was correct
pub fn solve_puzzle(cli: Cli) {
    let puzzle_input = match get_input(cli.year, cli.day) {
        Ok(input) => {
            info!("Got puzzle input for year {:?} day {:?}", cli.year, cli.day);
            input
        },
        Err(error) => panic!("Failed to get puzzle input! ({:?})", error)
    };
    
    let solution = get_puzzle_solution(cli.year, cli.day, cli.part, puzzle_input);

    match cli.method {
        Method::Solve => info!("Solution: {solution:?}"),
        Method::Submit => {
            let solution_result = post_puzzle_answer(
                cli.year,
                cli.day,
                cli.part,
                solution);
            info!("Submitted puzzle answer");
            info!("{}", solution_result.unwrap());
        }
    }
}

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
    let mut solutions: HashMap<
        (u16, u8, u8), Box<dyn Fn(String) -> String>> = HashMap::new();

    solutions.insert((2024, 1, 1), Box::new(year2024::day1::part1));
    solutions.insert((2024, 1, 2), Box::new(year2024::day1::part2));
    solutions.insert((2024, 2, 1), Box::new(year2024::day2::part1));
    solutions.insert((2024, 2, 2), Box::new(year2024::day2::part2));

    solutions.get(&(year, day, part)).unwrap()(input)
}
