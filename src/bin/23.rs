use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut board) = input_parser(input).unwrap();

    let mut dirs = VecDeque::from_iter([North, South, West, East]);
    for _ in 0..10 {
        move_elves(&mut board, &dirs);
        dirs.rotate_left(1);
    }

    let mut result = 0;
    let ((min_x, max_x), (min_y, max_y)) = board.get_bounds();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !board.cells.contains(&Pos { x, y }) {
                result += 1;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut board) = input_parser(input).unwrap();

    let mut dirs = VecDeque::from_iter([North, South, West, East]);

    let mut result = 0;
    while move_elves(&mut board, &dirs) {
        result += 1;
        dirs.rotate_left(1);
    }
    Some(result + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}
use crate::Dir::*;

impl Dir {
    fn check_positions(&self) -> Vec<Pos> {
        match self {
            North => vec![
                Pos { x: -1, y: -1 },
                Pos { x: 0, y: -1 },
                Pos { x: 1, y: -1 },
            ],
            South => vec![Pos { x: -1, y: 1 }, Pos { x: 0, y: 1 }, Pos { x: 1, y: 1 }],
            West => vec![
                Pos { x: -1, y: -1 },
                Pos { x: -1, y: 0 },
                Pos { x: -1, y: 1 },
            ],
            East => vec![Pos { x: 1, y: -1 }, Pos { x: 1, y: 0 }, Pos { x: 1, y: 1 }],
        }
    }

    fn step(&self) -> Pos {
        match self {
            North => Pos { x: 0, y: -1 },
            South => Pos { x: 0, y: 1 },
            West => Pos { x: -1, y: 0 },
            East => Pos { x: 1, y: 0 },
        }
    }
}

fn move_elves(board: &mut Board, dirs: &VecDeque<Dir>) -> bool {
    let mut proposed_moves: FxHashMap<Pos, Vec<Pos>> = FxHashMap::default();
    for elf in board.cells.iter() {
        if elf.neighbours().iter().all(|p| !board.cells.contains(p)) {
            continue;
        }

        for dir in dirs.iter() {
            let all_empty = dir
                .check_positions()
                .iter()
                .all(|step| !board.cells.contains(&(*elf + *step)));

            let proposed = *elf + dir.step();

            if all_empty {
                if let Some(old) = proposed_moves.get_mut(&proposed) {
                    old.push(*elf);
                } else {
                    proposed_moves.insert(proposed, vec![*elf]);
                }

                break;
            }
        }
    }

    if proposed_moves.is_empty() {
        return false;
    }

    let moves = proposed_moves.iter().filter_map(|(pos, elves)| {
        if elves.len() == 1 {
            Some((pos, elves[0]))
        } else {
            None
        }
    });

    for (dest, src) in moves {
        board.cells.remove(&src);
        board.cells.insert(*dest);
    }

    true
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        vec![
            Pos { x: -1, y: -1 },
            Pos { x: 0, y: -1 },
            Pos { x: 1, y: -1 },
            Pos { x: -1, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: -1, y: 1 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 1 },
        ]
        .iter()
        .map(|step| *self + *step)
        .collect()
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(PartialEq, Debug)]
struct Board {
    cells: FxHashSet<Pos>,
}

impl Board {
    fn get_bounds(&self) -> ((i32, i32), (i32, i32)) {
        let (all_xs, all_ys): (Vec<i32>, Vec<i32>) = self.cells.iter().map(|p| (p.x, p.y)).unzip();

        if let (MinMax(&min_x, &max_x), MinMax(&min_y, &max_y)) =
            (all_xs.iter().minmax(), all_ys.iter().minmax())
        {
            ((min_x, max_x), (min_y, max_y))
        } else {
            panic!("could not find bounds")
        }
    }

    fn _print(&self) {
        let ((min_x, max_x), (min_y, max_y)) = self.get_bounds();

        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                if self.cells.contains(&(Pos { x, y })) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!()
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn input_parser(i: &str) -> IResult<&str, Board> {
    all_consuming(terminated(board_parser, opt(tag("\n"))))(i)
}

fn board_parser(i: &str) -> IResult<&str, Board> {
    map(
        separated_list1(tag("\n"), many1(alt((tag("."), tag("#"))))),
        |rows: Vec<Vec<&str>>| {
            let mut result = FxHashSet::default();

            for (y, row) in rows.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };

                    if *c == "#" {
                        result.insert(pos);
                    }
                }
            }

            Board { cells: result }
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
