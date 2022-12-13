use parse_display::{Display, FromStr};

enum CraneBehaviour {
    OneByOne,
    AllTogether,
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut board_state, instructions) = parse_input(input);
    run_instructions(&mut board_state, instructions, CraneBehaviour::OneByOne);
    Some(board_state.iter().map(|stack| stack[0]).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut board_state, instructions) = parse_input(input);
    run_instructions(&mut board_state, instructions, CraneBehaviour::AllTogether);
    Some(board_state.iter().map(|stack| stack[0]).collect())
}

fn run_instructions(
    board_state: &mut BoardState,
    instructions: Vec<Instruction>,
    crane_mode: CraneBehaviour,
) {
    for instruction in instructions {
        let from = instruction.from - 1;
        let to = instruction.to - 1;

        // source stack
        let source: Vec<char> = board_state[from].clone();
        let (to_move, to_keep) = source.split_at(instruction.count);
        board_state[from] = to_keep.to_vec();

        // destination stack
        let mut result = to_move.to_vec();
        match &crane_mode {
            CraneBehaviour::OneByOne => result.reverse(),
            CraneBehaviour::AllTogether => (),
        }
        result.extend(board_state[to].clone());
        board_state[to] = result;
    }
}

type BoardState = Vec<Vec<char>>;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("[{value}]")]
struct StackValue {
    value: char,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {count} from {from} to {to}")]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (BoardState, Vec<Instruction>) {
    let (board_input, ins_input) = input.split_once("\n\n").unwrap();
    let lines: Vec<&str> = board_input.lines().collect();

    // create stacks
    let (columns_input, stacks_input) = lines.split_last().unwrap();
    let columns_count: usize = columns_input.split_whitespace().count();
    let mut columns = vec![vec![]; columns_count];

    // fill stacks
    for row_input in stacks_input {
        let by_column: Vec<String> = row_input
            .chars()
            .collect::<Vec<_>>()
            .chunks(4)
            .map(|chunk| { chunk.iter().collect::<String>().trim().to_owned() })
            .collect();

        for (index, column) in by_column.iter().enumerate() {
            if let Ok(elem) = column.parse::<StackValue>() {
                columns[index].push(elem.value)
            }
        }
    }

    // parse instructions
    let instructions = ins_input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    (columns, instructions)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::read_file("examples", 5);
        println!("{:?}", parse_input(&input));
        assert_eq!(1, 1);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
