pub fn part_one(input: &str) -> Option<(Vec<Coords>, usize)> {
    let board = Board::from(input);

    run_dijkstra(
        &[board.start],
        |node| board.successors(node),
        |node| node == &board.end,
    )
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

    run_dijkstra(
        &starting_cells,
        |node| board.successors(node),
        |node| node == &board.end,
    )
    .map(|(_, count)| count)
}

use eframe::egui;
use egui::{Color32, Sense, Stroke};
use std::collections::HashSet;
use std::time::Duration;

// fn main() {
//     let input = advent_of_code::read_file("inputs", 12);
//     advent_of_code::solve!(1, part_one, input);
//     advent_of_code::solve!(2, part_two, input);
// }

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            // this is the id of the `<canvas>` element we have
            // in our `index.html`
            "canvas",
            web_options,
            Box::new(|_cc| Box::new(MyApp::new())),
        )
        .await
        .expect("failed to start eframe");
    });
}


#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "AoC 2022 - Day 12",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}

// egui stuff

struct MyApp {
    coords: Vec<Coords>,
    current: usize,
    visited: HashSet<Coords>,
}

impl MyApp {
    fn new() -> Self {
        let input = include_str!("../inputs/12.txt");
        let (coords, _) = part_one(input).unwrap();

        Self {
            coords,
            current: 0,
            visited: HashSet::new(),
        }
    }

    fn update_state(&mut self) {
        match self.coords.get(self.current) {
            Some(v) => {
                self.visited.insert(*v);
                self.current += 1;
            }
            None => return,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_state();

        egui::CentralPanel::default().show(ctx, |ui| {
            const CANVAS_WIDTH: f32 = 900.0;
            const CANVAS_HEIGHT: f32 = 900.0;
            const SIDE: f32 = 6.0;

            let painter_size = egui::vec2(CANVAS_WIDTH, CANVAS_HEIGHT);
            let (_res, painter) = ui.allocate_painter(painter_size, Sense::hover());

            let to_panel_pos =
                |pos: Coords| egui::vec2(pos.x as f32 * SIDE, pos.y as f32 * SIDE).to_pos2();

            let width = (CANVAS_WIDTH / SIDE).floor() as i32;
            let height = (CANVAS_HEIGHT / SIDE).floor() as i32;

            for x in 0..width {
                for y in 0..height {
                    let dot = Coords {
                        x: x as usize,
                        y: y as usize,
                    };
                    if !self.visited.contains(&dot) {
                        continue;
                    }
                    let color = if self.coords.last() == Some(&dot) {
                        Color32::GOLD
                    } else {
                        Color32::DARK_RED
                    };

                    let dot_pos = to_panel_pos(dot);
                    painter.circle_stroke(dot_pos, 1.0, Stroke::new(2.0, color));
                }
            }

            if let (Some(prev), Some(curr)) = (
                self.coords.get(self.current - 1),
                self.coords.get(self.current),
            ) {
                // paint the head
                let head_pos = to_panel_pos(*prev);
                painter.circle_stroke(head_pos, 2.0, Stroke::new(2.0, Color32::GREEN));

                // paint the tail
                let tail_pos = to_panel_pos(*curr);
                painter.circle_stroke(tail_pos, 2.0, Stroke::new(2.0, Color32::YELLOW));

                // paint an arrow from head to tail
                painter.arrow(
                    tail_pos,
                    head_pos - tail_pos,
                    Stroke::new(1.0, Color32::YELLOW),
                )
            }
        });

        ctx.request_repaint_after(Duration::from_millis(25));
    }
}

// actually solving the problem

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coords {
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

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq, Copy, Debug)]
struct HeapState<T> {
    node: T,
    cost: usize,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl<T: Eq> Ord for HeapState<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: Eq> PartialOrd for HeapState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use core::hash::Hash;
use std::marker::Copy;

fn run_dijkstra<T: Eq + Hash + Copy, S: FnMut(&T) -> Vec<(T, usize)>, E: FnMut(&T) -> bool>(
    starting: &[T],
    mut successors: S,
    mut is_end: E,
) -> Option<(Vec<T>, usize)> {
    let mut best: HashMap<T, usize> = HashMap::new();
    let mut parent: HashMap<T, T> = HashMap::new();
    let mut heap = BinaryHeap::new();

    for start in starting {
        best.insert(*start, 0);

        heap.push(HeapState {
            node: *start,
            cost: 0,
        });
    }

    while let Some(HeapState { node, cost }) = heap.pop() {
        if is_end(&node) {
            let mut path = vec![];
            let mut current = Some(&node);

            path.push(node);
            while let Some(prev) = current {
                path.push(*prev);
                current = parent.get(prev)
            }
            path.reverse();

            return Some((path, cost));
        }

        for (next, next_cost) in successors(&node) {
            let new_cost = cost + next_cost;

            let next_state = HeapState {
                node: next,
                cost: new_cost,
            };

            if new_cost < *best.get(&next).unwrap_or(&std::usize::MAX) {
                best.insert(next, new_cost);
                parent.insert(next, node);
                heap.push(next_state);
            }
        }
    }

    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse() {
//         use pretty_assertions::assert_eq;
//         let input = advent_of_code::read_file("examples", 12);
//         let board = Board::from(&input[..]);

//         assert_eq!(board.start, Coords { x: 0, y: 0 });
//         assert_eq!(board.end, Coords { x: 5, y: 2 });
//         assert_eq!(board.num_rows, 5);
//         assert_eq!(board.num_cols, 8);
//         assert_eq!(board.to_string(), input.trim());
//     }

//     #[test]
//     fn test_part_one() {
//         let input = advent_of_code::read_file("examples", 12);
//         assert_eq!(part_one(&input).map(|x| x.1), Some(31));
//     }

//     #[test]
//     fn test_part_two() {
//         let input = advent_of_code::read_file("examples", 12);
//         assert_eq!(part_two(&input), Some(29));
//     }
// }
