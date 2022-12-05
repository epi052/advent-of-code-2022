use advent_of_code_2022::{parse_args, PuzzlePart};
use color_eyre::Result;

use std::collections::VecDeque;

static INPUT: &str = include_str!("../inputs/input-5");

/// parsed bracket offset to stack offset
fn whitespace_to_stack(spaces: usize) -> usize {
    // each bracket is found at these offsets from the first closing bracket 1, 5, 9, 13, 17, ...
    let mut stack = 0;

    (1..spaces).for_each(|space| {
        if space % 4 == 1 {
            stack += 1;
        }
    });

    stack
}

fn solve(input: &str, part: PuzzlePart) -> Result<String> {
    // 9 stacks of crates, deque used so we can parse from top-down
    // and still maintain a stack-like structure
    let mut stacks = Vec::with_capacity(9);
    (0..9).for_each(|_| stacks.push(VecDeque::new()));

    let (crates, instructions) = input.split_once("\n\n").unwrap();

    // populate crate stacks
    crates.lines().for_each(|line| {
        let mut tmp_ch = ' ';
        let mut tmp_stack = 0;

        line.char_indices().for_each(|(idx, ch)| {
            if ch == '[' {
                tmp_ch = ch;
                tmp_stack = whitespace_to_stack(idx);
            } else if tmp_ch == '[' {
                stacks[tmp_stack].push_back(ch);
                tmp_ch = ' ';
            }
        });
    });

    // do the things
    instructions.lines().for_each(|line| {
        // move 6 from 4 to 3
        let parts = line.split(' ').collect::<Vec<_>>();
        let num_to_move = parts[1].parse::<usize>().unwrap();
        let from_stack = parts[3].parse::<usize>().unwrap() - 1;
        let to_stack = parts[5].parse::<usize>().unwrap() - 1;
        match part {
            PuzzlePart::One => {
                (0..num_to_move).for_each(|_| {
                    // top-down parsing created the deque, so we can use the front
                    // as our stack top
                    let src_crate = stacks[from_stack].pop_front().unwrap();
                    stacks[to_stack].push_front(src_crate);
                });
            }
            PuzzlePart::Two => {
                let mut crate_group = stacks[from_stack]
                    .drain(..num_to_move)
                    .collect::<VecDeque<_>>();

                while let Some(src_crate) = crate_group.pop_back() {
                    stacks[to_stack].push_front(src_crate);
                }
            }
        }
    });

    let result = stacks.iter_mut().fold(String::new(), |mut acc, stack| {
        if let Some(top) = stack.pop_front() {
            acc.push(top);
        }
        acc
    });

    Ok(result)
}

fn part_one(input: &str) -> Result<String> {
    solve(input, PuzzlePart::One)
}

fn part_two(input: &str) -> Result<String> {
    solve(input, PuzzlePart::Two)
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
    static TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(String::from("CMZ"), part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(String::from("MCD"), part_two(TEST_INPUT)?);
        Ok(())
    }
}
