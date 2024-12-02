mod cli;
pub use cli::{Cli, Method};

mod web;
pub use web::{get_input, post_puzzle_answer};

mod puzzles;
pub use puzzles::solve_puzzle;
pub use puzzles::year2024;
