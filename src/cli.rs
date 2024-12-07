use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Which mode to run the program in (solve or submit)
    #[arg(value_enum)]
    pub method: Method,
    
    /// The AoC year
    pub year: u16,

    /// The day in December to solve puzzles for
    pub day: u8,

    /// Solve the second part of the day's puzzle (default: solve the first part)
    #[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..3))]
    pub part: u8,

    #[arg(short, long, default_value_t = false)]
    pub bench: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    /// Solve the puzzle and display the answer
    Solve,

    /// Solve the puzzle, submit the answer, and display either success or failure
    Submit,
}
