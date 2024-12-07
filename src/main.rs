use env_logger::{Builder, Env};
use clap::Parser;

use aoc_rs_1npo::solve_puzzle;
use aoc_rs_1npo::Cli;

fn main() {
    let cli = Cli::parse();
    let env = Env::default()
        .filter_or("AOC_LOG_LEVEL", "info")
        .write_style_or("AOC_LOG_STYLE", "always");
    let mut builder = Builder::from_env(env);

    builder.format_timestamp_micros().init();
    solve_puzzle(cli);
}
