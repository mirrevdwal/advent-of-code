use nom::{
    branch::alt, character::complete, combinator::map, multi::separated_list0, sequence::delimited,
    IResult,
};
use std::fs::read_to_string;

#[derive(Clone, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Number(u32),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Item::Number(x), Item::Number(y)) => x.partial_cmp(y),
            (Item::List(x), Item::List(y)) => x.partial_cmp(y),
            (Item::List(_), Item::Number(_)) => self.partial_cmp(&Item::List(vec![other.clone()])),
            (Item::Number(_), Item::List(_)) => Item::List(vec![self.clone()]).partial_cmp(other),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let data = parse_data("data/13.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let chunks = data.split("\n\n");

    let mut count = 0;
    for (index, chunk) in chunks.enumerate() {
        let mut pair = chunk.lines();

        let left = pair
            .next()
            .expect("Could not find first line of packet pair");
        let right = pair
            .next()
            .expect("Could not find first line of packet pair");

        let (_res, left) = parse_item(left).expect("Could not parse first packet");
        let (_res, right) = parse_item(right).expect("Could not parse second packet");

        if left < right {
            count += index + 1;
        }
    }

    count
}

fn part_two(data: &str) -> usize {
    let mut packets = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_item(line).unwrap().1)
        .collect::<Vec<_>>();

    let (_res, divider1) = parse_item("[[2]]").expect("Could not parse first divider packet");
    let (_res, divider2) = parse_item("[[6]]").expect("Could not parse second divider packet");

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    let mut packets = packets.iter();

    let first_position = packets
        .by_ref()
        .position(|packet| *packet == divider1)
        .unwrap()
        + 1;
    let second_position = packets
        .by_ref()
        .position(|packet| *packet == divider2)
        .unwrap()
        + first_position
        + 1;

    first_position * second_position
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((parse_number, parse_list))(input)
}

fn parse_number(input: &str) -> IResult<&str, Item> {
    map(complete::u32, Item::Number)(input)
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    map(
        delimited(
            complete::char('['),
            separated_list0(complete::char(','), parse_item),
            complete::char(']'),
        ),
        Item::List,
    )(input)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/13-example.txt");
        assert_eq!(part_one(&data), 13);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/13-example.txt");
        assert_eq!(part_two(&data), 140);
    }
}
