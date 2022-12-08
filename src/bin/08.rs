use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse_input(input);
    // print_board(&board);

    let (n_rows, n_cols) = get_dimensions(&board);
    let mut visible = vec![vec![0; n_cols]; n_rows];
    fill_borders(&mut visible);
    // print_board(&visible);

    fill_visible(&board, &mut visible);
    // print_board(&visible);

    Some(count_visible(&visible) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse_input(input);
    // print_board(&board);

    let (n_rows, n_cols) = get_dimensions(&board);
    let mut scores = vec![vec![0; n_cols]; n_rows];

    fill_scores(&board, &mut scores);
    // print_board(&scores);

    highest_score(&scores)
}

type Board = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Board {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect()
}

fn get_dimensions(board: &Board) -> (usize, usize) {
    let n_cols = board[0].len();
    let n_rows = board.len();
    (n_rows, n_cols)
}

fn _print_board(board: &Board) {
    for line in board {
        println!(
            "{}",
            line.iter()
                .fold(String::new(), |acc, e| acc + &e.to_string())
        );
    }
    println!()
}

fn fill_borders(board: &mut Board) {
    let (n_rows, n_cols) = get_dimensions(board);

    for x in 0..n_rows {
        board[0][x] = 1;
        board[n_cols - 1][x] = 1;
    }
    for x in 1..(n_cols - 1) {
        board[x][0] = 1;
        board[x][n_rows - 1] = 1;
    }
}

fn fill_visible(board: &Board, visible: &mut Board) {
    let (n_rows, n_cols) = get_dimensions(board);

    // top to bottom
    let mut max_top_to_bottom = board[0].clone();
    for x in 0..n_cols {
        for y in 0..n_rows {
            if board[y][x] > max_top_to_bottom[x] {
                max_top_to_bottom[x] = board[y][x];
                visible[y][x] = 1;
            }
        }
    }

    // bottom to top
    let mut max_bottom_to_top = board[n_rows - 1].clone();
    for x in 0..n_cols {
        for y in (0..n_rows).rev() {
            if board[y][x] > max_bottom_to_top[x] {
                max_bottom_to_top[x] = board[y][x];
                visible[y][x] = 1;
            }
        }
    }

    // left to right
    let mut max_left_to_right: Vec<u32> = board.iter().map(|row| row[0]).collect();
    for y in 0..n_rows {
        for x in 0..n_cols {
            if board[y][x] > max_left_to_right[y] {
                max_left_to_right[y] = board[y][x];
                visible[y][x] = 1;
            }
        }
    }

    // right to let
    let mut max_right_to_left: Vec<u32> = board.iter().map(|row| row[n_cols - 1]).collect();
    for x in (0..n_cols).rev() {
        for y in 0..n_rows {
            if board[y][x] > max_right_to_left[y] {
                max_right_to_left[y] = board[y][x];
                visible[y][x] = 1;
            }
        }
    }
}

fn count_visible(board: &Board) -> usize {
    board.iter().flatten().filter(|e| **e == 1).count()
}

fn fill_scores(board: &Board, scores: &mut Board) {
    let (n_rows, n_cols) = get_dimensions(board);

    for y in 0..n_rows {
        for x in 0..n_cols {
            let visible_up = (0..y)
                .rev()
                .fold_while(0, |acc, new_y| {
                    if board[new_y][x] < board[y][x] {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let visible_down = ((y + 1)..n_rows)
                .fold_while(0, |acc, new_y| {
                    if board[new_y][x] < board[y][x] {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let visible_left = (0..x)
                .rev()
                .fold_while(0, |acc, new_x| {
                    if board[y][new_x] < board[y][x] {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            let visible_right = ((x + 1)..n_cols)
                .fold_while(0, |acc, new_x| {
                    if board[y][new_x] < board[y][x] {
                        Continue(acc + 1)
                    } else {
                        Done(acc + 1)
                    }
                })
                .into_inner();

            scores[y][x] = visible_up * visible_down * visible_left * visible_right;
        }
    }
}

fn highest_score(board: &Board) -> Option<u32> {
    board.iter().flatten().max().copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "123\n456\n789";
        assert_eq!(
            parse_input(&input),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
