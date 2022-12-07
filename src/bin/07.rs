use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let commands: Vec<CommandWithOutput> = parse_input(input);
    let fs: FileSystem = build_filesystem(commands);
    let sizes: HashMap<Vec<&str>, u32> = calculate_sizes(fs);
    Some(sizes.values().filter(|value| **value < 100000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands: Vec<CommandWithOutput> = parse_input(input);
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
enum CommandWithOutput<'a> {
    Cd(Path<'a>),
    Ls(Vec<FsEntry<'a>>),
}

use crate::CommandWithOutput::*;

fn parse_input(input: &str) -> Vec<CommandWithOutput> {
    input
        .split("$ ")
        .filter_map(|chunk| match chunk.split_once('\n') {
            Some(("ls", output)) => Some(Ls(parse_fs_entries(output))),
            Some((other, _)) => match other.split_once(' ') {
                Some(("cd", "/")) => Some(Cd(Path::Root)),
                Some(("cd", "..")) => Some(Cd(Path::BackOne)),
                Some(("cd", other)) => Some(Cd(Path::Dir(other))),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

fn parse_fs_entries(input: &str) -> Vec<FsEntry> {
    input
        .lines()
        .filter_map(|line| match line.split_once(' ') {
            Some(("dir", dir_name)) => Some(FsEntry::Folder(dir_name)),
            Some((size, file)) => Some(FsEntry::File(file, size.parse().unwrap())),
            None => None,
        })
        .collect()
}

type FileSystem<'a> = HashMap<Vec<&'a str>, Vec<FsEntry<'a>>>;

fn build_filesystem(commands: Vec<CommandWithOutput>) -> FileSystem {
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
    fn test_parse_input() {
        let input = "$ cd /\n$ ls\ndir a\n123 g\n$ cd a\n$ cd ..\n";
        assert_eq!(
            parse_input(input),
            vec![
                Cd(Path::Root),
                Ls(vec![FsEntry::Folder("a"), FsEntry::File("g", 123)]),
                Cd(Path::Dir("a")),
                Cd(Path::BackOne)
            ]
        );
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
