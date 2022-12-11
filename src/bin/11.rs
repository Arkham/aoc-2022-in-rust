pub fn part_one(input: &str) -> Option<u64> {
    if let Ok(("", mut monkeys)) = monkeys_parser(input) {
        for _ in 0..20 {
            run_round(&mut monkeys, |x| x / 3);
        }

        find_score(&monkeys)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let Ok(("", mut monkeys)) = monkeys_parser(input) {
        for _ in 0..10000 {
            run_round(&mut monkeys, |x| x);
        }

        find_score(&monkeys)
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn _print_monkeys(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        println!(
            "Monkey {} ({}): {:?}",
            monkey.id, monkey.inspected_items, monkey.items
        );
    }
    println!()
}

fn find_score(monkeys: &Vec<Monkey>) -> Option<u64> {
    let mut counts = monkeys
        .iter()
        .map(|m| m.inspected_items)
        .collect::<Vec<_>>();

    counts.sort();
    counts.reverse();

    Some(counts[0] * counts[1])
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum OpArg {
    Old,
    Int(u64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum OpFun {
    Add,
    Mul,
}

type Op = (OpArg, OpFun, OpArg);

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    inspected_items: u64,
    op: Op,
    next: (u64, u64, u64),
}

fn run_round<F: FnOnce(u64) -> u64 + Copy>(monkeys: &mut Vec<Monkey>, after_run_op: F) {
    let modulus = monkeys.iter().fold(1, |a, m| a * m.next.0);

    for index in 0..monkeys.len() {
        let current = monkeys[index].clone();

        for item in current.items {
            let (rem_by, fst, snd) = current.next;
            let new_item = after_run_op(run_op(current.op, item) % modulus);
            let dest_monkey = if new_item % rem_by == 0 { fst } else { snd };
            monkeys[dest_monkey as usize].items.push(new_item);
        }

        monkeys[index].inspected_items += monkeys[index].items.len() as u64;
        monkeys[index].items = vec![];
    }
}

fn run_op(op: Op, old: u64) -> u64 {
    let fst = if OpArg::Old == op.0 {
        OpArg::Int(old)
    } else {
        op.0
    };
    let snd = if OpArg::Old == op.2 {
        OpArg::Int(old)
    } else {
        op.2
    };

    match (fst, op.1, snd) {
        (OpArg::Int(x), OpFun::Add, OpArg::Int(y)) => x + y,
        (OpArg::Int(x), OpFun::Mul, OpArg::Int(y)) => x * y,
        _ => panic!("run_op could not replace all args"),
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn monkeys_parser(input: &str) -> IResult<&str, Vec<Monkey>> {
    many1(terminated(monkey_parser, opt(tag("\n"))))(input)
}

fn int_parser(input: &str) -> IResult<&str, u64> {
    map(digit1, |s: &str| s.parse().unwrap())(input)
}

fn list_int_parser(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(", "), int_parser)(input)
}

fn op_parser(input: &str) -> IResult<&str, Op> {
    tuple((op_arg_parser, op_fun_parser, op_arg_parser))(input)
}

fn op_arg_parser(input: &str) -> IResult<&str, OpArg> {
    alt((
        map(tag("old"), |_| OpArg::Old),
        map(int_parser, |v| OpArg::Int(v)),
    ))(input)
}

fn op_fun_parser(input: &str) -> IResult<&str, OpFun> {
    alt((
        map(tag(" * "), |_| OpFun::Mul),
        map(tag(" + "), |_| OpFun::Add),
    ))(input)
}

fn next_parser(input: &str) -> IResult<&str, (u64, u64, u64)> {
    tuple((
        preceded(tag("  Test: divisible by "), int_parser),
        preceded(tag("\n    If true: throw to monkey "), int_parser),
        preceded(tag("\n    If false: throw to monkey "), int_parser),
    ))(input)
}

fn monkey_parser(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            terminated(preceded(tag("Monkey "), int_parser), tag(":\n")),
            terminated(
                preceded(tag("  Starting items: "), list_int_parser),
                tag("\n"),
            ),
            terminated(preceded(tag("  Operation: new = "), op_parser), tag("\n")),
            terminated(next_parser, tag("\n")),
        )),
        |(id, items, op, next)| Monkey {
            id: id,
            items: items,
            inspected_items: 0,
            op: op,
            next: next,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
