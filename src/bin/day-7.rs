use advent_of_code_2022::{parse_args, PuzzlePart};
use color_eyre::Result;

use std::collections::HashMap;

static INPUT: &str = include_str!("../inputs/input-7");

#[derive(Debug, Default)]
struct FsMap {
    cwd: Vec<String>,
    map: HashMap<String, i32>,
}

impl FsMap {
    fn from_history(input: &str) -> Self {
        let mut fs = FsMap::default();

        for line in input.lines() {
            let mut parts = line.split_whitespace();

            match line.chars().next() {
                Some('$') => {
                    // command
                    match parts.last() {
                        Some("/") => {
                            fs.cwd.clear();
                            fs.cwd.push("/".to_string());
                        }
                        Some("..") => {
                            fs.cwd.pop();
                        }
                        Some("ls") => {
                            // do nothing
                        }
                        Some(dir) => {
                            fs.cwd.push(dir.to_string());
                        }
                        None => continue,
                    }
                }
                Some('d') => {
                    // directory, do nothing
                }
                Some('1'..='9') => {
                    // file, add file size to all directories in cwd
                    let size = parts.next().unwrap().parse::<i32>().unwrap();
                    let mut tmp = fs.cwd.clone();

                    while !tmp.is_empty() {
                        *fs.map.entry(tmp.join("/")).or_insert(0) += size;
                        tmp.pop();
                    }
                }
                Some(_) | None => continue,
            }
        }

        fs
    }

    fn solve(&self, part: PuzzlePart) -> i32 {
        match part {
            PuzzlePart::One => self
                .map
                .iter()
                .filter_map(|(_, &size)| {
                    if size < 100_000 {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum(),
            PuzzlePart::Two => {
                let capacity = 70_000_000;
                let required = 30_000_000;
                let used = self.map.get("/").unwrap();
                let unused = capacity - used;
                let delta = required - unused;

                self.map
                    .iter()
                    .filter_map(|(_, &size)| {
                        if size >= delta {
                            Some(size)
                        } else {
                            None
                        }
                    })
                    .min()
                    .unwrap()
            }
        }
    }
}

fn part_one(input: &str) -> Result<i32> {
    let fs = FsMap::from_history(input);
    Ok(fs.solve(PuzzlePart::One))
}

fn part_two(input: &str) -> Result<i32> {
    let fs = FsMap::from_history(input);
    Ok(fs.solve(PuzzlePart::Two))
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
    static TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_one() -> Result<()> {
        assert_eq!(95437, part_one(TEST_INPUT)?);
        Ok(())
    }

    #[test]
    fn test_two() -> Result<()> {
        assert_eq!(24933642, part_two(TEST_INPUT)?);
        Ok(())
    }
}
