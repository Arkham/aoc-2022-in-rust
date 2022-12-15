use itertools::Itertools;
use parse_display::{Display, FromStr};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    part_one_internal(2_000_000, input)
}

pub fn part_two(input: &str) -> Option<i64> {
    part_two_internal(4_000_000, input)
}

fn part_one_internal(row: i32, input: &str) -> Option<i32> {
    let lines = parse_input(input);
    let board: Board = fill_board(lines);
    let beacons: Vec<Pos> = board
        .grid
        .iter()
        .filter_map(|(k, v)| if v == &Cell::Beacon { Some(k) } else { None })
        .copied()
        .collect();

    Some(
        build_ranges(row, &board.sensors)
            .iter()
            .map(|(start, end)| {
                let matching_beacons = beacons
                    .iter()
                    .filter(|pos| pos.y == row && (start..=end).contains(&&pos.x))
                    .count();
                end - start + 1 - (matching_beacons as i32)
            })
            .sum(),
    )
}

fn _part_two_internal_alt(limit: i32, input: &str) -> Option<i64> {
    let lines = parse_input(input);
    let board: Board = fill_board(lines);

    (0..=limit).into_par_iter().find_map_any(|y| {
        let ranges: Vec<(i32, i32)> = build_ranges(y, &board.sensors);
        // when we find two ranges, we found the hole
        if let [(_, before_result), (_, _)] = ranges[..] {
            Some((before_result + 1) as i64 * 4_000_000 + y as i64)
        } else {
            None
        }
    })
}

fn part_two_internal(limit: i32, input: &str) -> Option<i64> {
    let lines = parse_input(input);
    let board: Board = fill_board(lines);

    board.sensors.iter().find_map(|(sensor, radius)| {
        ((sensor.x - radius - 1)..=(sensor.x + radius + 1))
            .into_par_iter()
            .find_map_any(|x| {
                if x < 0 || x > limit {
                    return None;
                }

                let dy = radius - (x - sensor.x).abs() + 1;

                'outer: for y in [sensor.y + dy, sensor.y - dy] {
                    if y <= limit && y >= 0 {
                        // if there is at least one point which is not reachable
                        // from any other sensor, then it must be the solution
                        for (other, other_radius) in board.sensors.iter() {
                            if other.distance(Pos { x, y }) <= *other_radius {
                                break 'outer;
                            }
                        }
                        return Some(x as i64 * 4_000_000 + y as i64);
                    }
                }
                None
            })
    })
}

fn build_ranges(row: i32, sensors: &[(Pos, i32)]) -> Vec<(i32, i32)> {
    let sorted_ranges: Vec<(i32, i32)> = sensors
        .iter()
        .filter_map(|(sensor, radius)| {
            let distance = sensor.distance(Pos {
                x: sensor.x,
                y: row,
            });
            if distance <= *radius {
                let x_offset = radius - (sensor.y - row).abs();
                Some((sensor.x - x_offset, sensor.x + x_offset))
            } else {
                None
            }
        })
        .sorted_by_key(|e| e.0)
        .collect();

    let mut result: Vec<(i32, i32)> = vec![sorted_ranges[0]];
    for curr in sorted_ranges[1..].iter() {
        if let Some(&last) = &result.last() {
            if touch_or_overlap(&last, curr) {
                let new_range: (i32, i32) = (min(last.0, curr.0), max(last.1, curr.1));
                if new_range != last {
                    result.pop();
                    result.push(new_range)
                }
            } else {
                result.push(*curr)
            }
        }
    }
    result
}

fn touch_or_overlap((x1, x2): &(i32, i32), (y1, y2): &(i32, i32)) -> bool {
    !(x1 > y2 || y1 > &(x2 + 1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(PartialEq, Debug)]
enum Cell {
    Sensor,
    Beacon,
}

#[derive(PartialEq, Debug)]
struct Board {
    grid: HashMap<Pos, Cell>,
    sensors: Vec<(Pos, i32)>,
    bounds: ((i32, i32), (i32, i32)),
}

fn fill_board(info: Vec<SensorInfo>) -> Board {
    let mut grid = HashMap::new();
    let mut sensors = Vec::new();
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;

    for elem in info {
        grid.insert(elem.sensor, Cell::Sensor);
        grid.insert(elem.nearest_beacon, Cell::Beacon);
        let distance = elem.sensor.distance(elem.nearest_beacon);
        sensors.push((elem.sensor, distance));

        if elem.sensor.x - distance < min_x {
            min_x = elem.sensor.x - distance
        }

        if elem.sensor.x + distance > max_x {
            max_x = elem.sensor.x + distance
        }

        if elem.sensor.y - distance < min_y {
            min_y = elem.sensor.y - distance
        }

        if elem.sensor.y + distance > max_y {
            max_y = elem.sensor.y + distance
        }
    }

    Board {
        grid,
        sensors,
        bounds: ((min_x, max_x), (min_y, max_y)),
    }
}

fn _print_board(board: &Board) {
    let ((min_x, max_x), (min_y, max_y)) = board.bounds;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let row_ranges = build_ranges(y, &board.sensors);
            match board.grid.get(&Pos { x, y }) {
                Some(Cell::Sensor) => print!("S"),
                Some(Cell::Beacon) => print!("B"),
                None => {
                    if row_ranges
                        .iter()
                        .any(|(start, end)| *start <= x && x <= *end)
                    {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
            }
        }
        println!()
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Eq, Hash, Clone, Copy)]
#[display("x={x}, y={y}")]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn distance(self, other: Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at {sensor}: closest beacon is at {nearest_beacon}")]
struct SensorInfo {
    sensor: Pos,
    nearest_beacon: Pos,
}

fn parse_input(input: &str) -> Vec<SensorInfo> {
    input
        .lines()
        .map(|line| line.parse::<SensorInfo>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(
            parse_input(&input)[0],
            SensorInfo {
                sensor: Pos { x: 2, y: 18 },
                nearest_beacon: Pos { x: -2, y: 15 }
            }
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_internal(10, &input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_internal(20, &input), Some(56_000_011));
    }
}
