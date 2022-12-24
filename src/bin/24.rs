use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<usize> {
    let (_, board) = input_parser(input).unwrap();
    let mut cache = FxHashMap::from_iter([(0, board.clone())]);

    run_bfs(board.start, board.end, 0, &mut cache)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, board) = input_parser(input).unwrap();
    let mut cache = FxHashMap::from_iter([(0, board.clone())]);

    let there = run_bfs(board.start, board.end, 0, &mut cache)?;
    let back = run_bfs(board.end, board.start, there, &mut cache)?;
    run_bfs(board.start, board.end, back, &mut cache)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// hello bfs my old friend
fn run_bfs(
    start: Pos,
    end: Pos,
    initial_turn: usize,
    cache: &mut FxHashMap<usize, Board>,
) -> Option<usize> {
    let mut to_visit = VecDeque::new();
    let mut visited: FxHashSet<(Pos, usize)> = FxHashSet::default();

    to_visit.push_back((start, initial_turn));

    while let Some((node, cost)) = to_visit.pop_front() {
        if node == end {
            return Some(cost);
        }

        let next_cost = cost + 1;
        ensure_in_cache(cache, next_cost);

        for next in possible_moves(&node, &cache[&next_cost]) {
            if visited.contains(&(next, next_cost)) {
                continue;
            }
            visited.insert((next, next_cost));
            to_visit.push_back((next, next_cost));
        }
    }

    None
}

fn ensure_in_cache(cache: &mut FxHashMap<usize, Board>, turn: usize) {
    if cache.get(&turn).is_none() {
        let old_board = &cache[&(turn - 1)];
        let new_board = old_board.evolve();
        cache.insert(turn, new_board);
    }
}

fn possible_moves(pos: &Pos, board: &Board) -> Vec<Pos> {
    let (_, max_y) = board.bounds;

    pos.successors()
        .iter()
        .copied()
        .filter(|p| {
            p.y >= 0 && p.y <= max_y && !board.walls.contains(p) && !board.blizzards.contains_key(p)
        })
        .collect()
}

// data structures
#[derive(PartialEq, Debug, Clone, Copy)]
enum Blizzard {
    Upper,
    Downer,
    Lefter,
    Righter,
}

impl Blizzard {
    fn step(&self) -> Pos {
        let (x, y) = match self {
            Upper => (0, -1),
            Downer => (0, 1),
            Lefter => (-1, 0),
            Righter => (1, 0),
        };

        Pos { x, y }
    }
}

use std::fmt;

impl fmt::Display for Blizzard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Upper => "^",
                Downer => "v",
                Lefter => "<",
                Righter => ">",
            }
        )
    }
}

use crate::Blizzard::*;

#[derive(PartialEq, Debug, Clone)]
struct Board {
    start: Pos,
    end: Pos,
    bounds: (i32, i32),
    walls: FxHashSet<Pos>,
    blizzards: FxHashMap<Pos, Vec<Blizzard>>,
}

impl Board {
    fn evolve(&self) -> Self {
        let mut new_blizzs: FxHashMap<Pos, Vec<Blizzard>> = FxHashMap::default();

        for (pos, blizzs) in &self.blizzards {
            for blizz in blizzs {
                let new_blizz_pos = pos.add_and_wrap(blizz.step(), self.bounds);
                add_blizzard(&new_blizz_pos, *blizz, &mut new_blizzs);
            }
        }

        Board {
            blizzards: new_blizzs,
            walls: self.walls.clone(),
            start: self.start,
            end: self.end,
            bounds: self.bounds,
        }
    }

    fn _print(&self, player: &Pos) {
        let (max_x, max_y) = self.bounds;

        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos { x, y };
                if *player == pos {
                    print!("E");
                } else if self.walls.contains(&pos) {
                    print!("#");
                } else if let Some(v) = self.blizzards.get(&pos) {
                    if v.len() > 1 {
                        print!("{}", v.len());
                    } else {
                        print!("{}", v[0]);
                    }
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
        separated_list1(
            tag("\n"),
            many1(alt((
                tag("."),
                tag(">"),
                tag("<"),
                tag("^"),
                tag("v"),
                tag("#"),
            ))),
        ),
        |rows: Vec<Vec<&str>>| {
            let mut walls = FxHashSet::default();
            let mut blizzards = FxHashMap::default();

            for (y, row) in rows.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };

                    match *c {
                        "#" => {
                            walls.insert(pos);
                        }
                        ">" => {
                            add_blizzard(&pos, Righter, &mut blizzards);
                        }
                        "<" => {
                            add_blizzard(&pos, Lefter, &mut blizzards);
                        }
                        "^" => {
                            add_blizzard(&pos, Upper, &mut blizzards);
                        }
                        "v" => {
                            add_blizzard(&pos, Downer, &mut blizzards);
                        }
                        "." => (),
                        _ => panic!("did not expect this"),
                    }
                }
            }

            let max_x = rows[0].len() as i32 - 1;
            let max_y = rows.len() as i32 - 1;

            Board {
                start: Pos { x: 1, y: 0 },
                end: Pos {
                    x: max_x - 1,
                    y: max_y,
                },
                bounds: (max_x, max_y),
                walls,
                blizzards,
            }
        },
    )(i)
}

fn add_blizzard(pos: &Pos, blizzard: Blizzard, blizzards: &mut FxHashMap<Pos, Vec<Blizzard>>) {
    if let Some(old) = blizzards.get_mut(pos) {
        old.push(blizzard);
    } else {
        blizzards.insert(*pos, vec![blizzard]);
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add_and_wrap(&self, other: Self, bounds: (i32, i32)) -> Self {
        let (max_x, max_y) = bounds;
        let mut new = *self + other;

        if new.x < 1 {
            new.x = max_x - 1;
        }
        if new.x > max_x - 1 {
            new.x = 1;
        }
        if new.y < 1 {
            new.y = max_y - 1;
        }
        if new.y > max_y - 1 {
            new.y = 1;
        }

        new
    }

    fn successors(&self) -> Vec<Pos> {
        vec![
            Pos { x: 0, y: -1 }, // up
            Pos { x: 1, y: 0 },  // right
            Pos { x: 0, y: 1 },  // down
            Pos { x: -1, y: 0 }, // left
            Pos { x: 0, y: 0 },  // wait
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 24);
        assert!(input_parser(&input).is_ok());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
