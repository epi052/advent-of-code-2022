use std::collections::HashSet;

use advent_of_code_2022::parse_args;
use color_eyre::Result;

static INPUT: &str = include_str!("../inputs/input-6");

fn solve(input: &str, length: usize) -> Result<i32> {
    for chunk in input.char_indices().collect::<Vec<_>>().windows(length) {
        let unique = chunk.iter().map(|(_, ch)| ch).collect::<HashSet<_>>();

        if unique.len() == length {
            return Ok(chunk[0].0 as i32 + length as i32);
        }
    }

    Ok(0)
}

fn part_one(input: &str) -> Result<i32> {
    solve(input, 4)
}

fn part_two(input: &str) -> Result<i32> {
    solve(input, 14)
}

fn main() -> Result<()> {
    let args = parse_args();

    match args.part {
        1 => println!("{}", part_one(INPUT)?),
        2 => println!("{}", part_two(INPUT)?),
        _ => println!(
            "got unexpected value for --part: {} (try 1 or 2)",
            args.part
        ),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(7, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(19, part_two(TEST_INPUT)?);
        Ok(())
    }
}
