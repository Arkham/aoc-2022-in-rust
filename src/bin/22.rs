use rustc_hash::FxHashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let (_, (board, instructions)) = input_parser(input).unwrap();
    let mut player = Player {
        pos: board.start,
        dir: Dir::Right,
    };
    for ins in instructions {
        player = match ins {
            Ins::TurnClockwise => Player {
                dir: turn_clockwise(player.dir),
                pos: player.pos,
            },
            Ins::TurnCounterClockwise => Player {
                dir: turn_counter_clockwise(player.dir),
                pos: player.pos,
            },
            Ins::Forward(v) => move_player(v, &player, &board),
        }
    }
    Some(player_score(&player))
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, (board, instructions)) = input_parser(input).unwrap();
    let mut player = Player {
        pos: board.start,
        dir: Dir::Right,
    };
    for ins in instructions {
        player = match ins {
            Ins::TurnClockwise => Player {
                dir: turn_clockwise(player.dir),
                pos: player.pos,
            },
            Ins::TurnCounterClockwise => Player {
                dir: turn_counter_clockwise(player.dir),
                pos: player.pos,
            },
            Ins::Forward(v) => move_player_cube(v, &player, &board),
        }
    }
    Some(player_score(&player))
}

fn player_score(player: &Player) -> i32 {
    let row = (player.pos.y + 1) * 1000;
    let col = (player.pos.x + 1) * 4;
    let dir = match player.dir {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };
    row + col + dir
}

fn step_for_dir(dir: &Dir) -> Pos {
    let step = match dir {
        Left => (-1, 0),
        Up => (0, -1),
        Right => (1, 0),
        Down => (0, 1),
    };
    Pos {
        x: step.0,
        y: step.1,
    }
}

fn move_player(steps: u32, player: &Player, board: &Board) -> Player {
    let (max_x, max_y) = board.bounds;

    let step = step_for_dir(&player.dir);
    let mut current = player.pos;
    let mut next;
    let mut count = 0;

    while count < steps {
        count += 1;
        next = current + step;

        match board.cells.get(&next) {
            Some(Cell::Tile) => {
                current = next;
            }
            Some(Cell::Wall) => break,
            None => {
                let mut other_point = match player.dir {
                    Left => Pos {
                        x: max_x,
                        y: player.pos.y,
                    },
                    Right => Pos {
                        x: 0,
                        y: player.pos.y,
                    },
                    Up => Pos {
                        x: player.pos.x,
                        y: max_y,
                    },
                    Down => Pos {
                        x: player.pos.x,
                        y: 0,
                    },
                };

                while board.cells.get(&other_point).is_none() {
                    other_point += step;
                }
                if let Some(Cell::Tile) = board.cells.get(&other_point) {
                    current = other_point;
                } else {
                    break;
                }
            }
        }
    }

    Player {
        pos: current,
        dir: player.dir,
    }
}

fn move_player_cube(steps: u32, player: &Player, board: &Board) -> Player {
    let mut dir = player.dir;
    let mut pos = player.pos;
    let mut next;
    let mut count = 0;

    while count < steps {
        count += 1;

        let step = step_for_dir(&dir);
        next = pos + step;

        match board.cells.get(&next) {
            Some(Cell::Tile) => {
                pos = next;
            }
            Some(Cell::Wall) => break,
            None => {
                let (new_dir, other_point) = wrap_around(&dir, &pos);

                if let Some(Cell::Tile) = board.cells.get(&other_point) {
                    pos = other_point;
                    dir = new_dir;
                } else if board.cells.get(&other_point).is_none() {
                    panic!("this is wrong: {:?} -> {:?}", pos, other_point)
                } else {
                    break;
                }
            }
        }
    }

    Player { pos, dir }
}

fn wrap_around(dir: &Dir, pos: &Pos) -> (Dir, Pos) {
    match dir {
        Left => match pos.y {
            0..=49 => (
                Right,
                Pos {
                    x: 0,
                    y: (49 - pos.y) + 100,
                },
            ),
            50..=99 => (
                Down,
                Pos {
                    x: pos.y - 50,
                    y: 100,
                },
            ),
            100..=149 => (
                Right,
                Pos {
                    x: 50,
                    y: 49 - (pos.y - 100),
                },
            ),
            150..=199 => (
                Down,
                Pos {
                    x: (pos.y - 150) + 50,
                    y: 0,
                },
            ),
            _ => panic!("oopsies"),
        },
        Right => match pos.y {
            0..=49 => (
                Left,
                Pos {
                    x: 99,
                    y: 149 - pos.y,
                },
            ),
            50..=99 => (
                Up,
                Pos {
                    x: 100 + (pos.y - 50),
                    y: 49,
                },
            ),
            100..=149 => (
                Left,
                Pos {
                    x: 149,
                    y: 149 - pos.y,
                },
            ),
            150..=199 => (
                Up,
                Pos {
                    x: (pos.y - 150) + 50,
                    y: 149,
                },
            ),
            _ => panic!("oopsies"),
        },
        Up => match pos.x {
            0..=49 => (
                Right,
                Pos {
                    x: 50,
                    y: pos.x + 50,
                },
            ),
            50..=99 => (
                Right,
                Pos {
                    x: 0,
                    y: (pos.x - 50) + 150,
                },
            ),
            100..=149 => (
                Up,
                Pos {
                    x: pos.x - 100,
                    y: 199,
                },
            ),
            _ => panic!("oopsies"),
        },
        Down => match pos.x {
            0..=49 => (
                Down,
                Pos {
                    x: 100 + pos.x,
                    y: 0,
                },
            ),
            50..=99 => (
                Left,
                Pos {
                    x: 49,
                    y: (pos.x - 50) + 150,
                },
            ),
            100..=149 => (
                Left,
                Pos {
                    x: 99,
                    y: 50 + (pos.x - 100),
                },
            ),
            _ => panic!("oopsies"),
        },
    }
}

#[derive(PartialEq, Debug)]
struct Player {
    pos: Pos,
    dir: Dir,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(PartialEq, Debug)]
enum Cell {
    Tile,
    Wall,
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
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
    cells: FxHashMap<Pos, Cell>,
    start: Pos,
    bounds: (i32, i32),
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}
use crate::Dir::*;

#[derive(PartialEq, Debug)]
enum Ins {
    Forward(u32),
    TurnClockwise,
    TurnCounterClockwise,
}

fn turn_clockwise(dir: Dir) -> Dir {
    match dir {
        Left => Up,
        Up => Right,
        Right => Down,
        Down => Left,
    }
}
fn turn_counter_clockwise(dir: Dir) -> Dir {
    match dir {
        Left => Down,
        Up => Left,
        Right => Up,
        Down => Right,
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn input_parser(i: &str) -> IResult<&str, (Board, Vec<Ins>)> {
    all_consuming(terminated(
        separated_pair(board_parser, tag("\n\n"), ins_parser),
        opt(tag("\n")),
    ))(i)
}

fn board_parser(i: &str) -> IResult<&str, Board> {
    map(
        separated_list1(tag("\n"), many1(alt((tag(" "), tag("."), tag("#"))))),
        |rows: Vec<Vec<&str>>| {
            let mut result = FxHashMap::default();
            let mut first = None;
            let mut max_x = 0;
            let mut max_y = 0;

            for (y, row) in rows.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };

                    max_x = max_x.max(x as i32);
                    max_y = max_y.max(y as i32);

                    match *c {
                        "." => {
                            if first.is_none() {
                                first = Some(pos);
                            }
                            result.insert(pos, Cell::Tile);
                        }
                        "#" => {
                            result.insert(pos, Cell::Wall);
                        }
                        _ => (),
                    }
                }
            }

            Board {
                cells: result,
                start: first.unwrap(),
                bounds: (max_x, max_y),
            }
        },
    )(i)
}

fn ins_parser(i: &str) -> IResult<&str, Vec<Ins>> {
    many1(alt((
        map(tag("R"), |_| Ins::TurnClockwise),
        map(tag("L"), |_| Ins::TurnCounterClockwise),
        map(digit1, |v: &str| Ins::Forward(v.parse().unwrap())),
    )))(i)
}

fn _print_board(board: &Board, player: &Player) {
    let (max_x, max_y) = board.bounds;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if player.pos == (Pos { x, y }) {
                match player.dir {
                    Dir::Right => print!(">"),
                    Dir::Up => print!("^"),
                    Dir::Left => print!("<"),
                    Dir::Down => print!("v"),
                }
            } else {
                match board.cells.get(&Pos { x, y }) {
                    Some(Cell::Tile) => print!("."),
                    Some(Cell::Wall) => print!("#"),
                    None => print!(" "),
                }
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 22);
        assert!(input_parser(&input).is_ok());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }
}
