use std::cmp::Ordering;
use std::str::FromStr;

use advent_of_code_2022::parse_args;
use color_eyre::Result;

static INPUT: &str = include_str!("../inputs/input-2");

#[derive(Copy, Debug, PartialEq, Eq, Clone)]
enum HandShape {
    Rock(i32),
    Paper(i32),
    Scissors(i32),
}

impl HandShape {
    fn inner(&self) -> i32 {
        match self {
            HandShape::Rock(i) => *i,
            HandShape::Paper(i) => *i,
            HandShape::Scissors(i) => *i,
        }
    }

    fn versus(&self, other: &HandShape) -> i32 {
        match self.cmp(other) {
            Ordering::Less => self.inner(),
            Ordering::Equal => 3 + self.inner(),
            Ordering::Greater => 6 + self.inner(),
        }
    }
}

impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandShape::Rock(_), HandShape::Rock(_)) => Ordering::Equal,
            (HandShape::Rock(_), HandShape::Scissors(_)) => Ordering::Greater,
            (HandShape::Rock(_), HandShape::Paper(_)) => Ordering::Less,

            (HandShape::Paper(_), HandShape::Paper(_)) => Ordering::Equal,
            (HandShape::Paper(_), HandShape::Rock(_)) => Ordering::Greater,
            (HandShape::Paper(_), HandShape::Scissors(_)) => Ordering::Less,

            (HandShape::Scissors(_), HandShape::Scissors(_)) => Ordering::Equal,
            (HandShape::Scissors(_), HandShape::Paper(_)) => Ordering::Greater,
            (HandShape::Scissors(_), HandShape::Rock(_)) => Ordering::Less,
        }
    }
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for HandShape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" | "B X" | "A Y" | "C Z" => Ok(Self::Rock(1)),
            "B" | "Y" | "C X" | "B Y" | "A Z" => Ok(Self::Paper(2)),
            "C" | "Z" | "A X" | "C Y" | "B Z" => Ok(Self::Scissors(3)),
            _ => Err(()),
        }
    }
}

fn part_one(input: &str) -> Result<i32> {
    let results = input
        .split('\n')
        .map(|round| {
            let mut hands = round.split(' ').map(|s| s.parse::<HandShape>().unwrap());

            let opponent = hands.next().unwrap();
            let me = hands.next().unwrap();

            me.versus(&opponent)
        })
        .sum();

    Ok(results)
}

fn part_two(input: &str) -> Result<i32> {
    let results = input
        .split('\n')
        .map(|round| {
            let me = round.parse::<HandShape>().unwrap();
            let mut hands = round.split(' ').map(|s| s.parse::<HandShape>().unwrap());

            let opponent = hands.next().unwrap();

            me.versus(&opponent)
        })
        .sum();

    Ok(results)
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
    static TEST_INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(15, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(12, part_two(TEST_INPUT)?);
        Ok(())
    }
}
