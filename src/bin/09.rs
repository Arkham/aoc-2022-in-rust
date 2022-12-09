use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use parse_display::{Display, FromStr};
use rustc_hash::FxHashSet;

pub fn part_one(input: &str) -> Option<u32> {
    solve_with_size(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_with_size(input, 10)
}

fn solve_with_size(input: &str, size: usize) -> Option<u32> {
    let moves = parse_input(input);
    let mut state = initial_state(size);

    for ins in moves {
        run_instruction(&ins, &mut state);
    }
    // print_state(&state);

    Some(state.visited.len() as u32)
}

#[derive(Display, FromStr, PartialEq, Debug)]
enum Direction {
    #[display("R")]
    Right,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("U")]
    Up,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{dir} {moves}")]
struct Instruction {
    dir: Direction,
    moves: u8,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

type Coords = (i32, i32);

#[derive(PartialEq, Debug)]
struct State {
    rope: Vec<Coords>,
    visited: FxHashSet<Coords>,
}

fn initial_state(size: usize) -> State {
    State {
        rope: vec![(0, 0); size],
        visited: FxHashSet::from_iter([(0, 0)]),
    }
}

fn _print_state(state: &State) {
    let rope_as_set: FxHashSet<Coords> = state.rope.iter().copied().collect();
    let (all_xs, all_ys): (Vec<i32>, Vec<i32>) = state.visited.union(&rope_as_set).copied().unzip();

    if let (MinMax(min_x, max_x), MinMax(min_y, max_y)) =
        (all_xs.iter().minmax(), all_ys.iter().minmax())
    {
        for y in (*min_y..=*max_y).rev() {
            for x in *min_x..=*max_x {
                if rope_as_set.contains(&(x, y)) && state.visited.contains(&(x, y)) {
                    print!("O");
                } else if rope_as_set.contains(&(x, y)) {
                    print!("o");
                } else if state.visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn should_move(head: Coords, tail: Coords) -> Option<Coords> {
    let diff_x = head.0 - tail.0;
    let diff_y = head.1 - tail.1;

    match (diff_x, diff_y) {
        (-2, 0) | (2, 0) | (0, -2) | (0, 2) => Some((tail.0 + diff_x / 2, tail.1 + diff_y / 2)),
        (-1, -1) | (-1, 1) | (1, -1) | (1, 1) => None,
        (-1, 0) | (1, 0) | (0, -1) | (0, 1) => None,
        (0, 0) => None,
        _ => Some((
            tail.0 + diff_x / diff_x.abs(),
            tail.1 + diff_y / diff_y.abs(),
        )),
    }
}

fn splat_instruction(ins: &Instruction) -> (Coords, u8) {
    let step = match &ins.dir {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Right => (1, 0),
        Direction::Left => (-1, 0),
    };

    (step, ins.moves)
}

fn run_instruction(ins: &Instruction, state: &mut State) {
    let (step, times) = splat_instruction(ins);
    let rope_len = state.rope.len();

    for _ in 0..times {
        state.rope[0] = (state.rope[0].0 + step.0, state.rope[0].1 + step.1);

        for i in 1..rope_len {
            match should_move(state.rope[i - 1], state.rope[i]) {
                Some(new_knot) => {
                    state.rope[i] = new_knot;

                    if i == rope_len - 1 {
                        state.visited.insert(new_knot);
                    }
                }
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(
            parse_input(&input),
            vec![
                Instruction {
                    dir: Right,
                    moves: 4
                },
                Instruction { dir: Up, moves: 4 },
                Instruction {
                    dir: Left,
                    moves: 3
                },
                Instruction {
                    dir: Down,
                    moves: 1
                },
                Instruction {
                    dir: Right,
                    moves: 4
                },
                Instruction {
                    dir: Down,
                    moves: 1
                },
                Instruction {
                    dir: Left,
                    moves: 5
                },
                Instruction {
                    dir: Right,
                    moves: 2
                }
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_b() {
        let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(part_two(input), Some(36));
    }
}
