mod cli;
pub use cli::{Cli, Method};

mod web;
pub use web::{get_puzzle_input, post_puzzle_answer};

mod solver;
pub use solver::solve_puzzle;

mod puzzles;
pub use puzzles::year2024;
