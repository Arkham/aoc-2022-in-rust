use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (_, commands) = commands_parser(input).unwrap();
    let fs: FileSystem = build_filesystem(commands);
    let sizes: HashMap<Vec<&str>, u32> = calculate_sizes(fs);
    Some(sizes.values().filter(|value| **value < 100000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, commands) = commands_parser(input).unwrap();
    let fs: FileSystem = build_filesystem(commands);
    let sizes: HashMap<Vec<&str>, u32> = calculate_sizes(fs);
    let total: u32 = sizes[&vec!["/"]];
    let unused: u32 = total - 40000000;
    let mut in_order: Vec<u32> = sizes.into_values().collect();
    in_order.sort();
    in_order.iter().find(|el| **el > unused).copied()
}

#[derive(Debug, PartialEq)]
enum Path<'a> {
    Root,
    BackOne,
    Dir(&'a str),
}

#[derive(Debug, PartialEq)]
enum FsEntry<'a> {
    Folder(&'a str),
    File(&'a str, u32),
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    Cd(Path<'a>),
    Ls(Vec<FsEntry<'a>>),
}

use crate::Command::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, not_line_ending},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

fn commands_parser(input: &str) -> IResult<&str, Vec<Command>> {
    many1(terminated(alt((cd_parser, ls_parser)), opt(tag("\n"))))(input)
}

fn cd_parser(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
            tag("$ cd "),
            alt((
                map(tag("/"), |_| Path::Root),
                map(tag(".."), |_| Path::BackOne),
                map(not_line_ending, Path::Dir),
            )),
        ),
        Command::Cd,
    )(input)
}

fn ls_parser(input: &str) -> IResult<&str, Command> {
    map(
        preceded(tag("$ ls\n"), separated_list1(tag("\n"), ls_entry_parser)),
        Command::Ls,
    )(input)
}

fn ls_entry_parser(input: &str) -> IResult<&str, FsEntry> {
    alt((
        map(preceded(tag("dir "), not_line_ending), FsEntry::Folder),
        map(
            separated_pair(digit1, tag(" "), not_line_ending),
            |(file_size, file_name)| FsEntry::File(file_name, file_size.parse().unwrap()),
        ),
    ))(input)
}

type FileSystem<'a> = HashMap<Vec<&'a str>, Vec<FsEntry<'a>>>;

fn build_filesystem(commands: Vec<Command>) -> FileSystem {
    let mut result = HashMap::new();
    let mut current: Vec<&str> = vec!["/"];
    for command in commands {
        match command {
            Cd(Path::Root) => current = vec!["/"],
            Cd(Path::BackOne) => {
                if current != vec!["/"] {
                    current.pop();
                }
            }
            Cd(Path::Dir(dir)) => current.push(dir),
            Ls(output) => {
                result.insert(current.clone(), output);
            }
        }
    }
    result
}

fn calculate_sizes(fs: FileSystem) -> HashMap<Vec<&str>, u32> {
    let mut result: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut all_dirs: Vec<Vec<&str>> = fs.keys().cloned().collect();
    all_dirs.sort_by_key(|e| e.len());

    for dir in all_dirs.iter().rev() {
        let total: u32 = fs[dir]
            .iter()
            .map(|entry| match entry {
                FsEntry::Folder(v) => {
                    let mut new_path = dir.clone();
                    new_path.push(v);
                    result[&new_path]
                }
                FsEntry::File(_v, size) => *size,
            })
            .sum();
        result.insert(dir.clone(), total);
    }

    result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            commands_parser("$ ls\ndir a\n123 foo.txt\n$ cd /\n$ cd ..\n$ cd foo"),
            Ok((
                "",
                vec![
                    Command::Ls(vec![FsEntry::Folder("a"), FsEntry::File("foo.txt", 123)]),
                    Command::Cd(Path::Root),
                    Command::Cd(Path::BackOne),
                    Command::Cd(Path::Dir("foo")),
                ]
            ))
        )
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
