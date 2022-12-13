use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<usize> {
    let pairs: Vec<(Packet, Packet)> = pairs_parser(input).unwrap().1;

    Some(
        pairs
            .iter()
            .enumerate()
            .filter_map(|(index, (fst, snd))| match fst.cmp(snd) {
                Ordering::Less => Some(index + 1),
                _ => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut all_packets: Vec<Packet> = all_parser(input).unwrap().1;

    let two = List(vec![List(vec![Item(2)])]);
    let six = List(vec![List(vec![Item(6)])]);
    all_packets.push(two.clone());
    all_packets.push(six.clone());
    all_packets.sort();

    Some(
        all_packets
            .iter()
            .enumerate()
            .filter_map(|(index, v)| {
                if v == &two || v == &six {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Self>),
    Item(u32),
}

use crate::Packet::*;

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (List(_), Item(_)) => self.partial_cmp(&List(vec![other.clone()])),
            (Item(_), List(_)) => List(vec![self.clone()]).partial_cmp(other),
            (Item(x), Item(y)) => x.partial_cmp(y),
            (List(x), List(y)) => x.partial_cmp(y),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn pairs_parser(i: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    terminated(
        separated_list1(
            tag("\n\n"),
            separated_pair(packet_parser, tag("\n"), packet_parser),
        ),
        opt(tag("\n")),
    )(i)
}

fn all_parser(i: &str) -> IResult<&str, Vec<Packet>> {
    terminated(
        many1(terminated(packet_parser, alt((tag("\n\n"), tag("\n"))))),
        opt(tag("\n")),
    )(i)
}

fn packet_parser(i: &str) -> IResult<&str, Packet> {
    map(
        delimited(
            tag("["),
            separated_list0(tag(","), alt((item_parser, packet_parser))),
            tag("]"),
        ),
        Packet::List,
    )(i)
}

fn item_parser(i: &str) -> IResult<&str, Packet> {
    map(digit1, |s: &str| Packet::Item(s.parse().unwrap()))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        assert_eq!(
            packet_parser("[1,2,3]"),
            Ok(("", List(vec![Item(1), Item(2), Item(3)])))
        );

        assert_eq!(
            packet_parser("[1,[2,[4]],3]"),
            Ok((
                "",
                List(vec![
                    Item(1),
                    List(vec![Item(2), List(vec![Item(4)])]),
                    Item(3)
                ])
            ))
        );

        assert_eq!(
            pairs_parser("[1]\n[2]\n\n[3]\n[4]"),
            Ok((
                "",
                vec![
                    (List(vec![Item(1)]), List(vec![Item(2)])),
                    (List(vec![Item(3)]), List(vec![Item(4)])),
                ]
            ))
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
