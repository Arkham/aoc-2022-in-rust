use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let nums = parse_input(input);
    let num: Number = nums.iter().fold(Number::zero(), |acc, x| acc + x.clone());
    Some(format!("{}", num))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
}

#[derive(PartialEq, Eq, Debug, Display, FromStr, Clone, Copy)]
enum Digit {
    #[display("2")]
    Two,
    #[display("1")]
    One,
    #[display("0")]
    Zero,
    #[display("-")]
    Minus,
    #[display("=")]
    DoubleMinus,
}

use crate::Digit::*;

impl Digit {
    fn to_i32(self) -> i32 {
        match self {
            Two => 2,
            One => 1,
            Zero => 0,
            Minus => -1,
            DoubleMinus => -2,
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
struct Number {
    digits: Vec<Digit>,
}

impl Number {
    fn zero() -> Self {
        Self { digits: vec![Zero] }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.digits
                .iter()
                .map(|d| d.to_string())
                .collect::<String>()
        )
    }
}

fn parse_input(input: &str) -> Vec<Number> {
    input
        .lines()
        .map(|line| Number {
            digits: line
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect(),
        })
        .collect()
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let pairs: Vec<(Digit, Digit)> = rev_and_zip_all_digits(&self, &other);

        let mut result = VecDeque::new();

        let carry = pairs
            .iter()
            .fold(0, |acc, (x, y)| match x.to_i32() + y.to_i32() + acc {
                5 => {
                    result.push_front(Zero);
                    1
                }
                4 => {
                    // 4 is 5 - 1
                    result.push_front(Minus);
                    1
                }
                3 => {
                    // 3 is 5 - 2
                    result.push_front(DoubleMinus);
                    1
                }
                2 => {
                    result.push_front(Two);
                    0
                }
                1 => {
                    result.push_front(One);
                    0
                }
                0 => {
                    result.push_front(Zero);
                    0
                }
                -1 => {
                    result.push_front(Minus);
                    0
                }
                -2 => {
                    result.push_front(DoubleMinus);
                    0
                }
                -3 => {
                    // -3 is -5 + 2
                    result.push_front(Two);
                    -1
                }
                -4 => {
                    // -4 is -5 + 1
                    result.push_front(One);
                    -1
                }
                -5 => {
                    result.push_front(Zero);
                    -1
                }
                other => panic!("received {} when adding", other),
            });

        match carry {
            -1 => result.push_front(Minus),
            1 => result.push_front(One),
            _ => (),
        }

        Number {
            digits: result.iter().copied().collect(),
        }
    }
}

fn rev_and_zip_all_digits(fst: &Number, snd: &Number) -> Vec<(Digit, Digit)> {
    fst.digits
        .iter()
        .rev()
        .zip_longest(snd.digits.iter().rev())
        .map(|x| match x {
            Both(a, b) => (*a, *b),
            Left(a) => (*a, Zero),
            Right(b) => (Zero, *b),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! num [
        ($($e:expr),*) => {
            Number { digits: vec![$($e),*] }
        }
    ];

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(
            parse_input(&input),
            [
                num![One, DoubleMinus, Minus, Zero, Minus, Two],
                num![One, Two, One, One, One],
                num![Two, DoubleMinus, Zero, DoubleMinus],
                num![Two, One],
                num![Two, DoubleMinus, Zero, One],
                num![One, One, One],
                num![Two, Zero, Zero, One, Two],
                num![One, One, Two],
                num![One, DoubleMinus, Minus, One, DoubleMinus],
                num![One, Minus, One, Two],
                num![One, Two],
                num![One, DoubleMinus],
                num![One, Two, Two]
            ]
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(num![One] + num![One, Two], num![Two, DoubleMinus])
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".into()));
    }
}
