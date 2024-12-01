use log::info;

use crate::{Cli, Method, get_puzzle_input, post_puzzle_answer};
use crate::puzzles::get_puzzle_solution;

/// 1. Get the puzzle input for the given year and day (eg 2024, 1)
/// 2. Run the function that solves the given part of the puzzle (ie 1 or 2)
/// 3. Return the puzzle results using the given method:
///     - Solve: Print the puzzle solution
///     - Submit: Submit the puzzle solution to adventofcode.com and print a message
///         indicating whether or not the solution was correct
pub fn solve_puzzle(cli: Cli) {
    let puzzle_input = match get_puzzle_input(cli.year, cli.day) {
        Ok(input) => {
            info!("Got puzzle input for year {:?} day {:?} part {:?}",
                cli.year,
                cli.day,
                cli.part);
            input
        },
        Err(error) => panic!("Failed to get puzzle input! ({:?})", error)
    };
    
    let solution = get_puzzle_solution(cli.year, cli.day, cli.part, puzzle_input);

    match cli.method {
        Method::Solve => info!("Solution: {solution:?}"),
        Method::Submit => {
            let solution_result = post_puzzle_answer(cli.year,
                                                     cli.day,
                                                     cli.part,
                                                     solution);
            info!("Submitted puzzle answer");
            info!("{}", solution_result.unwrap());
        }
    }
}
