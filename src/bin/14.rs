use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use rustc_hash::FxHashMap;
use std::cmp::{max, min};

pub fn part_one(input: &str) -> Option<usize> {
    let (_, sequences) = input_parser(input).unwrap();

    let mut board: Board = build_board(sequences);
    fill_with_sand(&mut board);

    Some(board.grid.values().filter(|e| **e == Cell::Sand).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, sequences) = input_parser(input).unwrap();
    let mut board: Board = build_board(sequences);

    let ((min_x, max_x), (_, max_y)) = get_bounds(&board);
    for x in (min_x - 200)..(max_x + 200) {
        board.grid.insert((x, max_y + 2), Cell::Rock);
    }
    fill_with_sand(&mut board);

    Some(board.grid.values().filter(|e| **e == Cell::Sand).count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// main logic
fn fill_with_sand(board: &mut Board) {
    let (_, (_, max_y)) = get_bounds(board);

    let mut to_visit = Vec::new();
    let drop_point: Coords = (500, 0);
    to_visit.push(drop_point.clone());

    while let Some(v) = to_visit.last() {
        if v.1 < max_y {
            match neighbours(v).iter().find(|n| board.grid.get(n) == None) {
                Some(empty_tile) => to_visit.push(*empty_tile),
                None => {
                    if let Some(value) = to_visit.pop() {
                        board.grid.insert(value, Cell::Sand);
                        if value == drop_point {
                            break;
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}

fn neighbours(coords: &Coords) -> Vec<Coords> {
    let &(x, y) = coords;
    vec![(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
}

// data structures
type Coords = (i32, i32);

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Rock,
    Sand,
}

#[derive(Debug, PartialEq)]
struct Board {
    grid: FxHashMap<Coords, Cell>,
}

fn build_board(sequences: Vec<Vec<Coords>>) -> Board {
    let mut result = FxHashMap::default();

    for seq in sequences {
        let pairs: Vec<(_, _)> = seq.iter().zip(seq.iter().skip(1)).collect();
        for (fst, snd) in pairs {
            for to_add in coords_between(fst, snd) {
                result.insert(to_add, Cell::Rock);
            }
        }
    }

    Board { grid: result }
}

fn get_bounds(board: &Board) -> ((i32, i32), (i32, i32)) {
    let (all_xs, all_ys): (Vec<i32>, Vec<i32>) = board.grid.keys().copied().unzip();

    if let (MinMax(&min_x, &max_x), MinMax(&min_y, &max_y)) =
        (all_xs.iter().minmax(), all_ys.iter().minmax())
    {
        ((min_x, max_x), (min_y, max_y))
    } else {
        panic!("could not find bounds")
    }
}

fn _print_board(board: &Board) {
    let ((min_x, max_x), (min_y, max_y)) = get_bounds(board);

    for y in (min_y - 1)..=(max_y + 1) {
        for x in (min_x - 1)..=(max_x + 1) {
            match board.grid.get(&(x, y)) {
                Some(Cell::Rock) => print!("#"),
                Some(Cell::Sand) => print!("o"),
                None => print!("."),
            }
        }
        println!()
    }
    println!()
}

fn coords_between(fst: &Coords, snd: &Coords) -> Vec<Coords> {
    if fst.0 == snd.0 {
        (min(fst.1, snd.1)..=max(fst.1, snd.1))
            .map(|y_value| (fst.0, y_value))
            .collect()
    } else if fst.1 == snd.1 {
        (min(fst.0, snd.0)..=max(fst.0, snd.0))
            .map(|x_value| (x_value, fst.1))
            .collect()
    } else {
        panic!("point {:?} is not aligned with point {:?}", fst, snd)
    }
}

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

fn input_parser(i: &str) -> IResult<&str, Vec<Vec<Coords>>> {
    terminated(
        separated_list1(tag("\n"), separated_list1(tag(" -> "), coords_parser)),
        opt(tag("\n")),
    )(i)
}

fn coords_parser(i: &str) -> IResult<&str, Coords> {
    separated_pair(int_parser, tag(","), int_parser)(i)
}

fn int_parser(i: &str) -> IResult<&str, i32> {
    map(digit1, |s: &str| s.parse().unwrap())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(
            input_parser(&input),
            Ok((
                "",
                vec![
                    vec![(498, 4), (498, 6), (496, 6)],
                    vec![(503, 4), (502, 4), (502, 9), (494, 9)]
                ]
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
