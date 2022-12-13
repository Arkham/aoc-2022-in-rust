use std::collections::VecDeque;
use std::iter::FromIterator;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    find_unique_marker(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_unique_marker(input, 14)
}

fn find_unique_marker(input: &str, marker_length: usize) -> Option<u32> {
    let chars : Vec<char> = input.chars().collect();
    let mut window : VecDeque<char> = VecDeque::from_iter(chars[0..marker_length].to_owned());

    if all_different(&window) {
        return Some(marker_length as u32);
    }

    for (index, char) in chars[marker_length..].iter().enumerate() {
        window.pop_front();
        window.push_back(*char);
        if all_different(&window) {
            return Some(index as u32 + marker_length as u32 + 1);
        }
    }

    None
}

fn all_different(window: &VecDeque<char>) -> bool {
    window.iter().unique().count() == window.len()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_one(input), Some(7));
    }

    #[test]
    fn test_part_one_a() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(input), Some(5));
    }

    #[test]
    fn test_part_one_b() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(input), Some(6));
    }

    #[test]
    fn test_part_one_c() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_two(input), Some(19));
    }
}
