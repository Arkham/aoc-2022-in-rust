use parse_display::{Display, FromStr};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_input(input);
    Some(
        blueprints
            .into_par_iter()
            .map(|b| b.id * calculate_max_geodes(24, b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints: Vec<BlueprintInfo> = parse_input(input);
    Some(
        blueprints
            .into_par_iter()
            .take(3)
            .map(|b| calculate_max_geodes(32, b))
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("Blueprint {id}: Each ore robot costs {ore_robot_ore} ore. Each clay robot costs {clay_robot_ore} ore. Each obsidian robot costs {obsidian_robot_ore} ore and {obsidian_robot_clay} clay. Each geode robot costs {geode_robot_ore} ore and {geode_robot_obsidian} obsidian.")]
struct BlueprintInfo {
    id: u32,
    ore_robot_ore: u32,
    clay_robot_ore: u32,
    obsidian_robot_ore: u32,
    obsidian_robot_clay: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32,
}

fn parse_input(input: &str) -> Vec<BlueprintInfo> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct State {
    resources: Resources,
    robots: Robots,
}

fn calculate_max_geodes(time_limit: u32, bp: BlueprintInfo) -> u32 {
    let mut visited: FxHashSet<State> = FxHashSet::default();
    let mut best = 0;

    let mut to_visit = VecDeque::new();
    let initial_state = State {
        resources: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };
    to_visit.push_back((initial_state, 0));

    while let Some((mut current, turn)) = to_visit.pop_front() {
        if turn == time_limit {
            best = best.max(current.resources.geode);
            continue;
        }

        cap(&mut current, &bp, time_limit - turn);

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        if should_build_geode_robot(&current, &bp) {
            let mut new_state = collect_resources(&current);
            new_state.resources.ore -= bp.geode_robot_ore;
            new_state.resources.obsidian -= bp.geode_robot_obsidian;
            new_state.robots.geode += 1;
            to_visit.push_back((new_state, turn + 1));
            continue;
        }

        if should_build_obsidian_robot(&current, &bp) {
            let mut new_state = collect_resources(&current);
            new_state.resources.ore -= bp.obsidian_robot_ore;
            new_state.resources.clay -= bp.obsidian_robot_clay;
            new_state.robots.obsidian += 1;
            to_visit.push_back((new_state, turn + 1));
        }

        if should_build_clay_robot(&current, &bp) {
            let mut new_state = collect_resources(&current);
            new_state.resources.ore -= bp.clay_robot_ore;
            new_state.robots.clay += 1;
            to_visit.push_back((new_state, turn + 1));
        }

        if should_build_ore_robot(&current, &bp) {
            let mut new_state = collect_resources(&current);
            new_state.resources.ore -= bp.ore_robot_ore;
            new_state.robots.ore += 1;
            to_visit.push_back((new_state, turn + 1));
        }

        to_visit.push_back((collect_resources(&current), turn + 1))
    }

    best
}

fn collect_resources(state: &State) -> State {
    let State { resources, robots } = state;

    State {
        resources: Resources {
            ore: resources.ore + robots.ore,
            clay: resources.clay + robots.clay,
            obsidian: resources.obsidian + robots.obsidian,
            geode: resources.geode + robots.geode,
        },
        robots: *robots,
    }
}

fn should_build_geode_robot(state: &State, bp: &BlueprintInfo) -> bool {
    state.resources.ore >= bp.geode_robot_ore && state.resources.obsidian >= bp.geode_robot_obsidian
}

fn should_build_obsidian_robot(state: &State, bp: &BlueprintInfo) -> bool {
    state.robots.obsidian < bp.geode_robot_obsidian
        && state.resources.ore >= bp.obsidian_robot_ore
        && state.resources.clay >= bp.obsidian_robot_clay
}

fn should_build_clay_robot(state: &State, bp: &BlueprintInfo) -> bool {
    state.robots.clay < bp.obsidian_robot_clay && state.resources.ore >= bp.clay_robot_ore
}

fn max_ore(bp: &BlueprintInfo) -> u32 {
    *[
        bp.ore_robot_ore,
        bp.clay_robot_ore,
        bp.obsidian_robot_ore,
        bp.geode_robot_ore,
    ]
    .iter()
    .max()
    .unwrap()
}

fn should_build_ore_robot(state: &State, bp: &BlueprintInfo) -> bool {
    state.robots.ore < max_ore(bp) && state.resources.ore >= bp.ore_robot_ore
}

use std::cmp::min;

fn cap(state: &mut State, bp: &BlueprintInfo, time_left: u32) {
    // let's say that:
    // - an obsidian robot costs 4 clay and we have 3 clay robots
    // - there are 11 turns left before the end

    // in those 11 turns our 3 clay robots will produce 30 clays,
    // since we need the material at the beginning of the turn.
    // so if we have a reserve of at least 14 clays, we don't need
    // anymore clay and we can cap the production.

    // the formula is then: cost + (cost - n_robots) * (time_left - 1)
    // if we plug it in: 4 + (4 - 3) * 10 = 14

    // if the following turn we ended up creating a new clay robot,
    // the formula becomes: 4 + (4 - 4) * 9 = 4
    // this is because every turn we'll be generating exactly the
    // right amount of clay that we need. JIT baby!
    let min_formula = |cost, n_robots| cost + (cost - n_robots) * (time_left - 1);

    state.resources.ore = min(
        state.resources.ore,
        min_formula(max_ore(bp), state.robots.ore),
    );
    state.resources.clay = min(
        state.resources.clay,
        min_formula(bp.obsidian_robot_clay, state.robots.clay),
    );
    state.resources.obsidian = min(
        state.resources.obsidian,
        min_formula(bp.geode_robot_obsidian, state.robots.obsidian),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        let result = calculate_max_geodes(
            24,
            BlueprintInfo {
                id: 1,
                ore_robot_ore: 4,
                clay_robot_ore: 2,
                obsidian_robot_ore: 3,
                obsidian_robot_clay: 14,
                geode_robot_ore: 2,
                geode_robot_obsidian: 7,
            },
        );
        assert_eq!(result, 9)
    }

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(
            parse_input(&input),
            [
                BlueprintInfo {
                    id: 1,
                    ore_robot_ore: 4,
                    clay_robot_ore: 2,
                    obsidian_robot_ore: 3,
                    obsidian_robot_clay: 14,
                    geode_robot_ore: 2,
                    geode_robot_obsidian: 7
                },
                BlueprintInfo {
                    id: 2,
                    ore_robot_ore: 2,
                    clay_robot_ore: 3,
                    obsidian_robot_ore: 3,
                    obsidian_robot_clay: 8,
                    geode_robot_ore: 3,
                    geode_robot_obsidian: 12
                }
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 19);
    //     assert_eq!(part_two(&input), Some(1));
    // }
}
