use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i64> {
    let initial = parse_input(input);
    let mut mixed = mix_sequence(initial, 1);
    find_solution(&mut mixed)
}

pub fn part_two(input: &str) -> Option<i64> {
    let initial = parse_input(input).iter().map(|v| v * 811589153).collect();
    let mut mixed = mix_sequence(initial, 10);
    find_solution(&mut mixed)
}

fn find_solution(nums: &mut VecDeque<i64>) -> Option<i64> {
    let zero_pos = nums.iter().position(|v| *v == 0).unwrap();
    nums.rotate_left(zero_pos);

    Some(
        [1000, 2000, 3000]
            .iter()
            .map(|i| nums[i % nums.len()])
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn parse_input(input: &str) -> VecDeque<i64> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn mix_sequence(input: VecDeque<i64>, times: u8) -> VecDeque<i64> {
    let len = input.len() as i64;
    let mut result = VecDeque::from_iter(input.iter().copied().enumerate());

    for _ in 0..times {
        for (id, value) in input.iter().enumerate() {
            if *value == 0 {
                continue;
            }

            let idx = result.iter().position(|(r_id, _)| *r_id == id).unwrap();
            result.remove(idx);

            let new_idx = (idx as i64 + value) % (len - 1);
            let abs_idx = if new_idx < 0 {
                (len - 1 + new_idx) % (len - 1)
            } else {
                new_idx
            };
            result.insert(abs_idx as usize, (id, *value));
        }
    }

    result.iter().map(|(_, v)| *v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(parse_input(&input), [1, 2, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
