use advent_of_code_2022::parse_args;
use color_eyre::Result;

static INPUT: &str = include_str!("../inputs/input-3");

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part_one(input: &str) -> Result<i32> {
    let result = input
        .split("\n")
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let Some(shared) = left
                .chars()
                .filter(|l| {
                    right.contains(*l)
                }).next() else {
                    return 0;
                };

            ALPHABET.find(shared).unwrap() as i32 + 1
        })
        .sum();

    Ok(result)
}

fn part_two(input: &str) -> Result<i32> {
    // iterate over input lines in chunks of 3
    let result = input
        .split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let Some(shared) = group[0]
                    .chars()
                    .filter(|item| {
                        group[1].contains(*item) && group[2].contains(*item)
                    }).next() else {
                        return 0;
                    };
            ALPHABET.find(shared).unwrap() as i32 + 1
        })
        .sum::<i32>();

    Ok(result)
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
    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(157, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(70, part_two(TEST_INPUT)?);
        Ok(())
    }
}
