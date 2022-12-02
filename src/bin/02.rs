#[derive(Debug, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Draw,
    Loss,
}

use crate::HandShape::*;
use crate::RoundResult::*;

fn parse_hand_shape(input: &str) -> HandShape {
    match input {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("I couldn't match this input: {}", input),
    }
}

fn parse_result(input: &str) -> RoundResult {
    match input {
        "X" => Loss,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("I couldn't match this input: {}", input),
    }
}

fn compute_result(first: &HandShape, second: &HandShape) -> RoundResult {
    match (first, second) {
        (Rock, Rock) => Draw,
        (Paper, Paper) => Draw,
        (Scissors, Scissors) => Draw,
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Scissors, Paper) => Win,
        _ => Loss,
    }
}

fn compute_score(hand_shape: &HandShape, result: &RoundResult) -> u32 {
    let hand_score = match hand_shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let result_score = match result {
        Win => 6,
        Draw => 3,
        Loss => 0,
    };

    hand_score + result_score
}

fn find_shape(opponent: &HandShape, desired_outcome: &RoundResult) -> HandShape {
    match (opponent, desired_outcome) {
        (_, Draw) => opponent.clone(),
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|(x, y)| {
                let opponent = &parse_hand_shape(x);
                let mine = &parse_hand_shape(y);
                compute_score(mine, &compute_result(mine, opponent))
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .map(|(x, y)| {
                let opponent = &parse_hand_shape(x);
                let result = &parse_result(y);
                let mine = &find_shape(opponent, result);
                compute_score(mine, result)
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
