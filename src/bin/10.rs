use itertools::Itertools;
use parse_display::{Display, FromStr};

pub fn part_one(input: &str) -> Option<i32> {
    let result = run_input(input, 1);
    let desired_indexes = vec![20, 60, 100, 140, 180, 220];

    Some(
        desired_indexes
            .iter()
            .fold(0, |acc, &idx| match result.get(idx - 1) {
                Some((_ins, v)) => v * (idx as i32) + acc,
                None => acc,
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = run_input(input, 1);
    let mut screen_state: Vec<bool> = vec![];

    for (index, (_, x_value)) in result.iter().enumerate() {
        let sprite_range = (x_value - 1)..=(x_value + 1);
        let normalized_index = index as i32 % 40;
        if sprite_range.contains(&normalized_index) {
            screen_state.push(true);
        } else {
            screen_state.push(false);
        }
    }

    print_screen(&screen_state);

    Some(screen_state.iter().filter(|e| **e).count() as u32)
}

fn run_input(input: &str, initial: i32) -> Vec<(Instruction, i32)> {
    let mut register_x: i32 = initial;
    let instructions = parse_input(input);
    let mut result = vec![(Instruction::NoOp, register_x)];

    for ins in instructions {
        match ins {
            Instruction::NoOp => result.push((ins, register_x)),
            Instruction::AddX(v) => {
                result.push((ins, register_x));
                register_x += v;
                result.push((ins, register_x));
            }
        }
    }

    result
}

fn print_screen(screen: &[bool]) {
    let rows = screen.iter().map(|v| if *v { '#' } else { ' ' }).chunks(40);
    for row in &rows {
        let result = row.collect::<String>();

        if result.len() == 40 {
            println!("{}", result);
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy, PartialOrd)]
enum Instruction {
    #[display("noop")]
    NoOp,
    #[display("addx {0}")]
    AddX(i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.parse().expect("did not find instruction"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction::*;

    #[test]
    fn test_parse() {
        let input = "noop\naddx 3\naddx -5";
        assert_eq!(parse_input(input), vec![NoOp, AddX(3), AddX(-5)]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two_short() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1";
        assert_eq!(part_two(input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(124));
    }
}
