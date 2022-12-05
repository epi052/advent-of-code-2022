use advent_of_code_2022::{parse_args, PuzzlePart};
use color_eyre::Result;

use std::collections::HashSet;
use std::sync::Once;

static INPUT: &str = include_str!("../inputs/input-4");
static INIT: Once = Once::new();
static mut REGEX: Option<regex::Regex> = None;

fn pattern() -> &'static regex::Regex {
    unsafe {
        INIT.call_once(|| {
            REGEX = Some(regex::Regex::new("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap());
        });
        REGEX.as_ref().unwrap()
    }
}

fn solve(input: &str, part: PuzzlePart) -> i32 {
    input
        .lines()
        .filter(|line| {
            let captures = pattern().captures(line).unwrap();

            let elves = [1, 3]
                .into_iter()
                .map(|idx| {
                    let min = captures[idx].parse::<i32>().unwrap();
                    let max = captures[idx + 1].parse::<i32>().unwrap();

                    (min..=max).collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>();

            match part {
                PuzzlePart::One => elves[0].is_subset(&elves[1]) || elves[0].is_superset(&elves[1]),
                PuzzlePart::Two => elves[0].intersection(&elves[1]).next().is_some(),
            }
        })
        .count() as i32
}

fn part_one(input: &str) -> Result<i32> {
    Ok(solve(input, PuzzlePart::One))
}

fn part_two(input: &str) -> Result<i32> {
    Ok(solve(input, PuzzlePart::Two))
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
    static TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(2, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(4, part_two(TEST_INPUT)?);
        Ok(())
    }
}
