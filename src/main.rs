use env_logger::Env;
use clap::Parser;

use aoc_rs_1npo::solve_puzzle;
use aoc_rs_1npo::Cli;

fn main() {
    let cli = Cli::parse();
    let env = Env::default()
        .filter_or("AOC_LOG_LEVEL", "info")
        .write_style_or("AOC_LOG_STYLE", "always");
    
    env_logger::init_from_env(env);
    solve_puzzle(cli);
}
