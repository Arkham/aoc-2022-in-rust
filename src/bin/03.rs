use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input_one(input)
            .iter()
            .map(|char| char_to_value(char))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input_two(input)
            .iter()
            .map(|char| char_to_value(char))
            .sum(),
    )
}

fn char_to_value(input: &char) -> u32 {
    let ascii = *input as u32;
    if ascii >= 97 {
        // Lowercase item types a through z have priorities 1 through 26
        ascii - 96
    } else {
        // Uppercase item types A through Z have priorities 27 through 52
        ascii - 64 + 26
    }
}

fn parse_input_one(input: &str) -> Vec<char> {
    input
        .lines()
        .flat_map(|line| {
            let (first, second) = line.split_at(line.chars().count() / 2);
            let first_set: HashSet<char> = HashSet::from_iter(first.chars());
            let second_set: HashSet<char> = HashSet::from_iter(second.chars());
            Vec::from_iter(&first_set & &second_set)
        })
        .collect::<Vec<_>>()
}

fn parse_input_two(input: &str) -> Vec<char> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|group| {
            let first_set: HashSet<char> = HashSet::from_iter(group[0].chars());
            let second_set: HashSet<char> = HashSet::from_iter(group[1].chars());
            let third_set: HashSet<char> = HashSet::from_iter(group[2].chars());
            Vec::from_iter(&(&first_set & &second_set) & &third_set)
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_one() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(parse_input_one(input), vec!['p']);
    }

    #[test]
    fn test_parse_input_two() {
        let input =
            "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg";
        assert_eq!(parse_input_two(input), vec!['r']);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
