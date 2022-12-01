use std::collections::BinaryHeap;

use advent_of_code_2022::parse_args;

static INPUT: &str = include_str!("../inputs/input-1");

fn part_one(input: &str) -> i32 {
    let mut results: BinaryHeap<_> = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calories| calories.parse::<i32>().unwrap())
                .sum()
        })
        .collect();

    results.pop().unwrap()
}

fn part_two(input: &str) -> i32 {
    let results = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calories| calories.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<BinaryHeap<_>>();

    results.iter().take(3).sum()
}

fn main() {
    let args = parse_args();

    match args.part {
        1 => println!("{}", part_one(INPUT)),
        2 => println!("{}", part_two(INPUT)),
        _ => println!(
            "got unexpected value for --part: {} (try 1 or 2)",
            args.part
        ),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_one() {
        assert_eq!(24000, part_one(TEST_INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(45000, part_two(TEST_INPUT));
    }
}
