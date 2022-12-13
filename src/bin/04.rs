pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .iter()
            .filter(|(x, y)| contain(*x, *y))
            .count()
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .iter()
            .filter(|(x, y)| overlap(*x, *y))
            .count()
    )
}

fn contain((x1, x2): (u32, u32), (y1, y2): (u32, u32)) -> bool {
    (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
}

fn overlap((x1, x2): (u32, u32), (y1, y2): (u32, u32)) -> bool {
    !(x1 > y2 || y1 > x2)
}

fn to_int(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

fn parse_input(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let (x1, x2) = x.split_once('-').unwrap();
            let (y1, y2) = y.split_once('-').unwrap();
            ((to_int(x1), to_int(x2)), (to_int(y1), to_int(y2)))
        })
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_input() {
        let input = "2-4,6-8";
        assert_eq!(parse_input(input), vec![((2, 4), (6, 8))]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
