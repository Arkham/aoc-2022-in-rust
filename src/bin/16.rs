use rustc_hash::{FxHashMap, FxHashSet};

pub fn part_one(input: &str) -> Option<u64> {
    let (_, valves) = valves_parser(input).unwrap();
    let valves_map: FxHashMap<&str, &Valve> = valves.iter().map(|v| (v.name, v)).collect();
    Some(find_best_throughput("AA", 30, &valves_map))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, valves) = valves_parser(input).unwrap();
    let valves_map: FxHashMap<&str, &Valve> = valves.iter().map(|v| (v.name, v)).collect();
    Some(find_with_elephant("AA", 26, &valves_map))
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct ToVisit<'a> {
    valve: &'a str,
    open_valves: FxHashSet<&'a str>,
    throughput: u64,
    turns: u32,
}

fn total_flow(open_valves: &FxHashSet<&str>, map: &FxHashMap<&str, &Valve>) -> u64 {
    open_valves.iter().map(|v| map[v].flow_rate).sum()
}

fn can_open_valve(
    valve: &str,
    open_valves: &FxHashSet<&str>,
    map: &FxHashMap<&str, &Valve>,
) -> bool {
    map[valve].flow_rate > 0 && !open_valves.contains(valve)
}

fn find_best_throughput(start: &str, turns_limit: u32, map: &FxHashMap<&str, &Valve>) -> u64 {
    let mut visited: FxHashMap<(&str, u32), u64> = FxHashMap::default();
    let mut to_visit = Vec::new();
    let mut best = 0;

    to_visit.push(ToVisit {
        valve: start,
        open_valves: FxHashSet::default(),
        throughput: 0,
        turns: 1,
    });

    while let Some(cur) = to_visit.pop() {
        if let Some(v) = visited.get(&(cur.valve, cur.turns)) {
            if v >= &cur.throughput {
                continue;
            }
        }

        visited.insert((cur.valve, cur.turns), cur.throughput);

        if cur.turns == turns_limit {
            best = std::cmp::max(best, cur.throughput);
            continue;
        }

        if can_open_valve(cur.valve, &cur.open_valves, map) {
            let mut new_valves = cur.open_valves.clone();
            new_valves.insert(cur.valve);

            to_visit.push(ToVisit {
                valve: cur.valve,
                throughput: cur.throughput + total_flow(&new_valves, map),
                open_valves: new_valves,
                turns: cur.turns + 1,
            });
        }

        for next in map[cur.valve].leads_to.iter() {
            to_visit.push(ToVisit {
                valve: next,
                throughput: cur.throughput + total_flow(&cur.open_valves, map),
                open_valves: cur.open_valves.clone(),
                turns: cur.turns + 1,
            });
        }
    }

    best
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct WithElephant<'a> {
    me: &'a str,
    elephant: &'a str,
    open_valves: FxHashSet<&'a str>,
    throughput: u64,
    turns: u32,
}

fn both_move<'a>(
    cur: WithElephant<'a>,
    map: &'a FxHashMap<&str, &Valve>,
    to_visit: &mut Vec<WithElephant<'a>>,
) {
    for next_me in map[cur.me].leads_to.iter() {
        for next_elephant in map[cur.elephant].leads_to.iter() {
            if next_me != next_elephant {
                to_visit.push(WithElephant {
                    me: next_me,
                    elephant: next_elephant,
                    throughput: cur.throughput + total_flow(&cur.open_valves, map),
                    open_valves: cur.open_valves.clone(),
                    turns: cur.turns + 1,
                });
            }
        }
    }
}

fn both_open_valve<'a>(
    cur: WithElephant<'a>,
    map: &'a FxHashMap<&str, &Valve>,
    to_visit: &mut Vec<WithElephant<'a>>,
) {
    let mut new_valves = cur.open_valves.clone();
    new_valves.insert(cur.me);
    new_valves.insert(cur.elephant);

    to_visit.push(WithElephant {
        me: cur.me,
        elephant: cur.elephant,
        throughput: cur.throughput + total_flow(&new_valves, map),
        open_valves: new_valves,
        turns: cur.turns + 1,
    })
}

fn i_open_valve<'a>(
    cur: WithElephant<'a>,
    map: &'a FxHashMap<&str, &Valve>,
    to_visit: &mut Vec<WithElephant<'a>>,
) {
    let mut new_valves = cur.open_valves.clone();
    new_valves.insert(cur.me);

    for next_elephant in map[cur.elephant].leads_to.iter() {
        to_visit.push(WithElephant {
            me: cur.me,
            elephant: next_elephant,
            throughput: cur.throughput + total_flow(&new_valves, map),
            open_valves: new_valves.clone(),
            turns: cur.turns + 1,
        });
    }
}

fn elephant_open_valve<'a>(
    cur: WithElephant<'a>,
    map: &'a FxHashMap<&str, &Valve>,
    to_visit: &mut Vec<WithElephant<'a>>,
) {
    let mut new_valves = cur.open_valves.clone();
    new_valves.insert(cur.elephant);

    for next_me in map[cur.me].leads_to.iter() {
        to_visit.push(WithElephant {
            me: next_me,
            elephant: cur.elephant,
            throughput: cur.throughput + total_flow(&new_valves, map),
            open_valves: new_valves.clone(),
            turns: cur.turns + 1,
        });
    }
}

fn find_with_elephant(start: &str, turns_limit: u32, map: &FxHashMap<&str, &Valve>) -> u64 {
    let mut visited: FxHashMap<(&str, &str, u32), u64> = FxHashMap::default();
    let mut to_visit = Vec::new();
    let mut best = 0;

    to_visit.push(WithElephant {
        me: start,
        elephant: start,
        open_valves: FxHashSet::default(),
        throughput: 0,
        turns: 1,
    });

    while let Some(cur) = to_visit.pop() {
        if let Some(v) = visited.get(&(cur.me, cur.elephant, cur.turns)) {
            if v >= &cur.throughput {
                continue;
            }
        }

        visited.insert((cur.me, cur.elephant, cur.turns), cur.throughput);

        if cur.turns == turns_limit {
            best = std::cmp::max(best, cur.throughput);
            continue;
        }

        // if all valves are open we can stop exploring
        if cur.open_valves.len() == map.values().filter(|v| v.flow_rate > 0).count() {
            to_visit.push(WithElephant {
                me: cur.me,
                elephant: cur.elephant,
                open_valves: cur.open_valves.clone(),
                throughput: cur.throughput
                    + ((turns_limit - cur.turns) as u64) * total_flow(&cur.open_valves, map),
                turns: turns_limit,
            });
            continue;
        }

        match (
            can_open_valve(cur.me, &cur.open_valves, map),
            can_open_valve(cur.elephant, &cur.open_valves, map),
        ) {
            (true, true) => {
                both_open_valve(cur.clone(), map, &mut to_visit);
                i_open_valve(cur.clone(), map, &mut to_visit);
                elephant_open_valve(cur.clone(), map, &mut to_visit);
                both_move(cur, map, &mut to_visit)
            }
            (true, false) => {
                i_open_valve(cur.clone(), map, &mut to_visit);
                both_move(cur, map, &mut to_visit);
            }
            (false, true) => {
                elephant_open_valve(cur.clone(), map, &mut to_visit);
                both_move(cur, map, &mut to_visit);
            }
            (false, false) => both_move(cur, map, &mut to_visit),
        }
    }

    best
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, PartialEq)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u64,
    leads_to: Vec<&'a str>,
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{all_consuming, map, opt},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn valves_parser(i: &str) -> IResult<&str, Vec<Valve>> {
    all_consuming(terminated(
        separated_list1(tag("\n"), valve_parser),
        opt(tag("\n")),
    ))(i)
}

fn valve_parser(i: &str) -> IResult<&str, Valve> {
    map(
        tuple((
            tag("Valve "),
            alphanumeric1,
            tag(" has flow rate="),
            digit1,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alphanumeric1),
        )),
        |(_, name, _, flow_rate, _, leads_to)| Valve {
            name,
            flow_rate: flow_rate.parse().unwrap(),
            leads_to,
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(valves_parser(&input).unwrap().0, "");
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 16);
    //     assert_eq!(part_two(&input), Some(1707));
    // }
}
