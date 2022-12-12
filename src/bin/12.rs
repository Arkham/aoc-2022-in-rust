use pathfinding::prelude::dijkstra;

pub fn part_one(input: &str) -> Option<usize> {
    let board = Board::from(input);

    dijkstra(&board.start, |p| board.successors(p), |p| p == &board.end).map(|(_, count)| count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let board = Board::from(input);

    let mut starting_cells: Vec<Coords> = vec![];

    for (y, row) in board.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Cell::Value('a') = cell {
                starting_cells.push(Coords { x, y })
            }
        }
    }

    starting_cells
        .iter()
        .filter_map(|start| {
            dijkstra(start, |p| board.successors(p), |p| p == &board.end).map(|(_, count)| count)
        })
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum Cell {
    Start,
    End,
    Value(char),
}

impl Cell {
    fn score(&self) -> i32 {
        match self {
            Cell::Start => 0,
            Cell::Value(c) => *c as i32 - 96, // from 1 to 26
            Cell::End => 27,
        }
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Start => "S".to_string(),
            Cell::End => "E".to_string(),
            Cell::Value(c) => format!("{}", c),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Board {
    num_rows: i32,
    num_cols: i32,
    start: Coords,
    end: Coords,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn successors(&self, coords: &Coords) -> Vec<(Coords, usize)> {
        let &Coords { x, y } = coords;
        let offsets: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        offsets
            .iter()
            .filter_map(|offset| {
                let (new_x, new_y) = (x as i32 + offset.0, y as i32 + offset.1);
                if (0..self.num_rows).contains(&new_y) && (0..self.num_cols).contains(&new_x) {
                    let new_score = self.cells[new_y as usize][new_x as usize].score();
                    let old_score = self.cells[y][x].score();

                    if new_score - old_score <= 1 {
                        Some((
                            Coords {
                                x: new_x as usize,
                                y: new_y as usize,
                            },
                            1,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let mut start: Coords = Coords { x: 0, y: 0 };
        let mut end: Coords = Coords { x: 0, y: 0 };

        let cells: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, elem)| match elem {
                        'S' => {
                            start = Coords { x, y };
                            Cell::Start
                        }
                        'E' => {
                            end = Coords { x, y };
                            Cell::End
                        }
                        other => Cell::Value(other),
                    })
                    .collect()
            })
            .collect();

        Board {
            num_rows: cells.len() as i32,
            num_cols: cells[0].len() as i32,
            start,
            end,
            cells,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use pretty_assertions::assert_eq;
        let input = advent_of_code::read_file("examples", 12);
        let board = Board::from(&input[..]);

        assert_eq!(board.start, Coords { x: 0, y: 0 });
        assert_eq!(board.end, Coords { x: 5, y: 2 });
        assert_eq!(board.num_rows, 5);
        assert_eq!(board.num_cols, 8);
        assert_eq!(board.to_string(), input.trim());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
