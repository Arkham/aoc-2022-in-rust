use parse_display::{Display, FromStr};
use rustc_hash::FxHashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let cubes = parse_input(input);
    let world = World::from_cubes(cubes);
    Some(world.count_visible_sides())
}

pub fn part_two(input: &str) -> Option<usize> {
    let cubes = parse_input(input);
    let world = World::from_cubes(cubes);
    Some(world.flow_and_count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

// data structures
#[derive(Display, FromStr, PartialEq, Debug, Eq, Hash, Clone, Copy)]
#[display("{x},{y},{z}")]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn neighbours(&self, bounds: (i32, i32, i32)) -> Vec<Cube> {
        let (max_x, max_y, max_z) = bounds;
        let offsets = vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];

        offsets
            .iter()
            .filter_map(|(dx, dy, dz)| {
                let new_x = self.x + dx;
                let new_y = self.y + dy;
                let new_z = self.z + dz;

                if (-1..=max_x + 1).contains(&new_x)
                    && (-1..=max_y + 1).contains(&new_y)
                    && (-1..=max_z + 1).contains(&new_z)
                {
                    Some(Cube {
                        x: new_x,
                        y: new_y,
                        z: new_z,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(PartialEq, Debug)]
struct World {
    cubes: FxHashSet<Cube>,
}

impl World {
    fn from_cubes(cubes: Vec<Cube>) -> Self {
        Self {
            cubes: FxHashSet::from_iter(cubes),
        }
    }

    fn get_bounds(&self) -> (i32, i32, i32) {
        let cubes = self.cubes.iter().collect::<Vec<_>>();
        let max_x = cubes.iter().map(|c| c.x).max().unwrap();
        let max_y = cubes.iter().map(|c| c.y).max().unwrap();
        let max_z = cubes.iter().map(|c| c.z).max().unwrap();
        (max_x, max_y, max_z)
    }

    fn count_visible_sides(&self) -> usize {
        let bounds = self.get_bounds();
        let mut count = 0;

        for cube in &self.cubes {
            for neighbour in cube.neighbours(bounds) {
                if !self.cubes.contains(&neighbour) {
                    count += 1;
                }
            }
        }
        count
    }

    fn flow_and_count(&self) -> usize {
        let bounds = &self.get_bounds();
        let start = Cube {
            x: -1,
            y: -1,
            z: -1,
        };

        // we start from the outside and we count cubes that we can reach,
        // in this way we don't count the empty air pockets inside.
        let mut count = 0;
        let mut visited = FxHashSet::default();
        let mut to_visit = Vec::new();
        to_visit.push(start);

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);
            for neighbour in current.neighbours(*bounds) {
                if self.cubes.contains(&neighbour) {
                    count += 1
                } else {
                    to_visit.push(neighbour)
                }
            }
        }

        count
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "1,1,1\n2,1,1";
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
