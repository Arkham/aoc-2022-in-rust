use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<i64> {
    let shapes = get_all_shapes();
    let directions = parse_input(input);
    let mut direction_source = directions.iter().cycle();

    let mut board = Board::new();
    let mut counter = 0;
    let threshold = 2022;

    for current_shape in shapes.iter().cycle() {
        counter += 1;
        if counter > threshold {
            break;
        }

        board.add_shape(current_shape.clone());

        for dir in direction_source.by_ref() {
            board.move_shape(*dir, 1);

            if board.move_shape(Direction::Down, 1) {
                continue;
            } else {
                board.settle_shape();
                break;
            }
        }
    }

    Some(board.height())
}

pub fn part_two(input: &str) -> Option<i64> {
    let shapes = get_all_shapes();
    let directions = parse_input(input);
    let mut direction_source = directions.iter().enumerate().cycle();

    let mut memory: FxHashMap<(i64, usize, usize), (u64, i64)> = FxHashMap::default();

    let mut board = Board::new();
    let mut counter: u64 = 0;
    let threshold: u64 = 1_000_000_000_000;

    for (shape_index, current_shape) in shapes.iter().enumerate().cycle() {
        counter += 1;
        if counter > threshold {
            break;
        }

        board.add_shape(current_shape.clone());

        for (dir_index, dir) in direction_source.by_ref() {
            board.move_shape(*dir, 1);

            if board.move_shape(Direction::Down, 1) {
                continue;
            } else {
                board.settle_shape();

                let board_hash = board.hash(8);

                // if we have a similar board with the same shape and same direction,
                // we have detected a cycle and can skip a lot of steps
                if let Some((old_counter, old_height)) =
                    memory.get(&(board_hash, shape_index, dir_index))
                {
                    let counter_diff = counter - old_counter;
                    let height_diff = board.height() - old_height;
                    let remaining = (threshold - counter) / counter_diff;
                    counter += remaining * counter_diff;
                    board.floor_level += remaining as i64 * height_diff;
                } else {
                    memory.insert(
                        (board_hash, shape_index, dir_index),
                        (counter, board.height()),
                    );
                }

                break;
            }
        }
    }

    Some(board.height())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, PartialEq)]
struct Board {
    current: Option<Shape>,
    points: FxHashSet<Pos>,
    floor_level: i64,
    max_y: i64, // cache height for performance
}

impl Board {
    pub fn new() -> Self {
        Self {
            current: None,
            points: (0..7).map(|x| Pos { x, y: 0 }).collect(),
            floor_level: 0,
            max_y: 0,
        }
    }

    fn add_shape(&mut self, shape: Shape) {
        self.current = Some(
            shape
                .shift(Direction::Right, 2)
                .shift(Direction::Up, self.max_y + 4),
        );
    }

    fn move_shape(&mut self, dir: Direction, count: i64) -> bool {
        if let Some(current) = &self.current {
            let next_position = current.shift(dir, count);
            let min_x = next_position.points.iter().map(|p| p.x).min().unwrap();
            let max_x = next_position.points.iter().map(|p| p.x).max().unwrap();

            if min_x >= 0
                && max_x < 7
                && self.points.intersection(&next_position.points).count() == 0
            {
                self.current = Some(next_position);
                return true;
            }
        }

        false
    }

    fn settle_shape(&mut self) {
        if let Some(current) = &self.current {
            let mut max_y = self.max_y;
            for point in &current.points {
                self.points.insert(point.clone());
                max_y = max_y.max(point.y);
            }
            self.current = None;
            self.max_y = max_y;

            self.shrink();
        }
    }

    fn shrink(&mut self) {
        let result: Option<i64> = (1..=self.max_y)
            .rev()
            .find(|y| (0..7).all(|x| self.points.get(&Pos { x, y: *y }).is_some()));
        if let Some(high_y) = result {
            self.points = self
                .points
                .iter()
                .filter_map(|pos| {
                    if pos.y >= high_y {
                        let difference = pos.y - high_y;
                        Some(Pos {
                            x: pos.x,
                            y: difference,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            self.floor_level += high_y;
            self.max_y -= high_y;
        }
    }

    fn hash(&self, top_rows: i64) -> i64 {
        let min_y = self.max_y - (top_rows - 1);
        let bools: Vec<bool> = (min_y..=self.max_y)
            .flat_map(|y| {
                (0..7)
                    .map(|x| self.points.get(&Pos { x, y }).is_some())
                    .collect::<Vec<_>>()
            })
            .collect();

        bools
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| if *v { (2 as i64).pow(i as u32) } else { 0 })
            .sum()
    }

    fn height(&self) -> i64 {
        self.floor_level + self.max_y
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut max_y = self.max_y;
        let max_y_shape = self
            .current
            .as_ref()
            .and_then(|c| c.points.iter().map(|p| p.y).max())
            .unwrap_or(0);

        max_y = max_y.max(max_y_shape);

        let result: String = (0..=max_y)
            .rev()
            .map(|y| {
                (0..7)
                    .map(|x| {
                        let cur_pos = Pos { x, y };

                        if y == 0 {
                            '-'
                        } else if self
                            .current
                            .as_ref()
                            .and_then(|c| c.points.get(&cur_pos))
                            .is_some()
                        {
                            '@'
                        } else if self.points.get(&cur_pos).is_some() {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", result)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            _ => None,
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Clone)]
struct Shape {
    points: FxHashSet<Pos>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseShapeError;

impl FromStr for Shape {
    type Err = ParseShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: FxHashSet<Pos> = FxHashSet::default();

        for (y, line) in s.lines().rev().enumerate() {
            for (x, value) in line.chars().enumerate() {
                if value == '#' {
                    result.insert(Pos {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }

        Ok(Shape { points: result })
    }
}

impl Shape {
    fn shift(&self, dir: Direction, count: i64) -> Self {
        let (step_x, step_y) = match dir {
            Direction::Right => (count, 0),
            Direction::Left => (-count, 0),
            Direction::Down => (0, -count),
            Direction::Up => (0, count),
        };

        Shape {
            points: self
                .points
                .iter()
                .map(|Pos { x, y }| Pos {
                    x: *x + step_x,
                    y: *y + step_y,
                })
                .collect(),
        }
    }
}

fn get_all_shapes() -> Vec<Shape> {
    let input = "\
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

    input
        .split("\n\n")
        .filter_map(|group| Shape::from_str(group).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction::*;

    #[test]
    fn test_parse_directions() {
        let input = "<<>>";
        assert_eq!(parse_input(input), vec![Left, Left, Right, Right]);
    }

    #[test]
    fn test_parse_shapes() {
        let input = "\
..#
..#
###
";
        assert_eq!(
            Shape::from_str(input),
            Ok(Shape {
                points: FxHashSet::from_iter(vec![
                    Pos { y: 0, x: 0 },
                    Pos { y: 0, x: 1 },
                    Pos { y: 0, x: 2 },
                    Pos { y: 1, x: 2 },
                    Pos { y: 2, x: 2 },
                ])
            })
        );
    }

    #[test]
    fn test_parse_all_shapes() {
        assert_eq!(get_all_shapes().len(), 5);
    }

    #[test]
    fn test_move_shape() {
        let input = "####";

        assert_eq!(
            Shape::from_str(input).map(|s| s.shift(Right, 1)),
            Ok(Shape {
                points: FxHashSet::from_iter(vec![
                    Pos { y: 0, x: 1 },
                    Pos { y: 0, x: 2 },
                    Pos { y: 0, x: 3 },
                    Pos { y: 0, x: 4 },
                ])
            })
        );
    }

    // #[test]
    // fn test_part_one() {
    //     let input = advent_of_code::read_file("examples", 17);
    //     assert_eq!(part_one(&input), Some(3068));
    // }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 17);
    //     assert_eq!(part_two(&input), Some(1514285714288));
    // }
}
