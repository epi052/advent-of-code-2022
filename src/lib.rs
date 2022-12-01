use clap::Parser;
use std::fs::read_to_string;
use std::path::Path;

/// cli for aoc binaries
#[derive(Parser)]
pub struct AoCArgParser {
    /// which part of the day
    #[clap(short, long)]
    pub part: i32,
}

/// parse cli arguments
pub fn parse_args() -> AoCArgParser {
    AoCArgParser::parse()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Solver {
    PartOne,
    PartTwo,
}
