use std::cell::RefCell;
use std::collections::HashMap;

type InsMap<'a> = HashMap<&'a str, RefCell<Value<'a>>>;

pub fn part_one(input: &str) -> Option<i64> {
    let (_, ins) = input_parser(input).unwrap();
    let map: InsMap = ins.iter().map(|i| (i.res, RefCell::new(i.val))).collect();
    Some(calculate_field("root", &map))
}

use std::process::Command;

pub fn part_two(path: &str) -> Option<i64> {
    let output = match Command::new("src/bin/21.sh").args([path]).output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(err) => panic!("Running process error: {}", err),
    };

    let res: f64 = output.parse().unwrap();
    Some(res.round() as i64)
}

fn run_op(fst: i64, op: Op, snd: i64) -> i64 {
    match op {
        Op::Add => fst + snd,
        Op::Sub => fst - snd,
        Op::Mul => fst * snd,
        Op::Div => fst / snd,
    }
}

fn calculate_field<'a>(field: &'a str, map: &InsMap) -> i64 {
    let result = match *map[field].borrow() {
        Value::Lit(v) => v,
        Value::Expr((fst, op, snd)) => {
            let x = calculate_field(fst, map);
            let y = calculate_field(snd, map);
            run_op(x, op, y)
        }
    };

    if *map[field].borrow() != Value::Lit(result) {
        *map[field].borrow_mut() = Value::Lit(result);
    }

    result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, "src/inputs/21.txt");
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Value<'a> {
    Lit(i64),
    Expr((&'a str, Op, &'a str)),
}

#[derive(PartialEq, Debug)]
struct Ins<'a> {
    res: &'a str,
    val: Value<'a>,
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{all_consuming, map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

fn input_parser(i: &str) -> IResult<&str, Vec<Ins>> {
    all_consuming(terminated(
        separated_list1(tag("\n"), ins_parser),
        opt(tag("\n")),
    ))(i)
}

fn value_parser(i: &str) -> IResult<&str, Value> {
    alt((
        map(digit1, |v: &str| Value::Lit(v.parse::<i64>().unwrap())),
        map(
            tuple((
                alphanumeric1,
                alt((
                    map(tag(" + "), |_| Op::Add),
                    map(tag(" - "), |_| Op::Sub),
                    map(tag(" * "), |_| Op::Mul),
                    map(tag(" / "), |_| Op::Div),
                )),
                alphanumeric1,
            )),
            Value::Expr,
        ),
    ))(i)
}

fn ins_parser(i: &str) -> IResult<&str, Ins> {
    map(
        separated_pair(alphanumeric1, tag(": "), value_parser),
        |(res, val)| Ins { res, val },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("src/examples/21.txt"), Some(301));
    }
}
