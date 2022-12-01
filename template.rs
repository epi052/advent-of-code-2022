use advent_of_code_2022::parse_args;

static INPUT: &str = include_str!("../inputs/input-DAY_NUM");

fn part_one(input: &str) -> i32 {
    0
}

fn part_two(input: &str) -> i32 {
    0
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
    static TEST_INPUT: &str = "";

    #[test]
    fn test_one() {
        assert_eq!(24000, part_one(TEST_INPUT));
    }

    #[test]
    fn test_two() {
        assert_eq!(0, part_two(TEST_INPUT));
    }
}
