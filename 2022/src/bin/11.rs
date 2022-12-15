use std::{collections::VecDeque, fs::read_to_string};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    sequence::tuple, IResult,
};

struct Monkey<'a> {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize + 'a>,
    test_divisible: usize,
    if_true: usize,
    if_false: usize,
}

impl<'a> TryFrom<&'a str> for Monkey<'a> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut lines = value.lines().skip(1);

        let (_name, items_str) = lines
            .next()
            .ok_or("No line found for starting items")?
            .split_once(": ")
            .ok_or("No values found for starting items")?;
        let items = items_str
            .split(", ")
            .filter_map(|item| item.parse::<usize>().ok())
            .collect::<VecDeque<_>>();

        let (_name, operation_str) = lines
            .next()
            .ok_or("No line found for operation")?
            .split_once(": ")
            .ok_or("No values found for operation")?;
        let (_lhs, rhs) = operation_str
            .split_once(" = ")
            .ok_or("Operation was not an algebraic operation")?;

        let (_res, operation) = parse_operation(rhs).map_err(|_| "Could not parse operation")?;

        let test_divisible = lines
            .next()
            .ok_or("No line found for test")?
            .split_whitespace()
            .last()
            .ok_or("Could not find test value")?
            .parse::<usize>()?;

        let if_true = lines
            .next()
            .ok_or("No line found for if-true")?
            .split_whitespace()
            .last()
            .ok_or("Could not find if-true value")?
            .parse::<usize>()?;

        let if_false = lines
            .next()
            .ok_or("No line found for if-false")?
            .split_whitespace()
            .last()
            .ok_or("Could not find if-false value")?
            .parse::<usize>()?;

        Ok(Monkey {
            items,
            operation,
            test_divisible,
            if_true,
            if_false,
        })
    }
}

fn main() {
    let data = parse_data("data/11.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(input: &str) -> usize {
    let segments = input.split("\n\n");

    let mut monkeys = segments
        .filter_map(|segment| Monkey::try_from(segment).ok())
        .collect::<Vec<_>>();

    let mut inspection_count = vec![0; monkeys.len()];
    let mut new_items = vec![Vec::new(); monkeys.len()];

    for _round in 0..20 {
        for (index, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.extend(new_items[index].drain(..));

            while let Some(item) = monkey.items.pop_front() {
                inspection_count[index] += 1;
                let new_item = (*monkey.operation)(item) / 3;
                if new_item % monkey.test_divisible == 0 {
                    new_items[monkey.if_true].push(new_item);
                } else {
                    new_items[monkey.if_false].push(new_item);
                }
            }
        }
    }

    inspection_count.sort();
    inspection_count.into_iter().rev().take(2).product()
}

fn part_two(input: &str) -> usize {
    let segments = input.split("\n\n");

    let mut monkeys = segments
        .filter_map(|segment| Monkey::try_from(segment).ok())
        .collect::<Vec<_>>();

    let mut inspection_count = vec![0; monkeys.len()];
    let mut new_items = vec![Vec::new(); monkeys.len()];

    let product = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible)
        .product::<usize>();

    for _round in 0..10_000 {
        for (index, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items.extend(new_items[index].drain(..));

            while let Some(item) = monkey.items.pop_front() {
                inspection_count[index] += 1;
                let new_item = (*monkey.operation)(item) % product;
                if new_item % monkey.test_divisible == 0 {
                    new_items[monkey.if_true].push(new_item);
                } else {
                    new_items[monkey.if_false].push(new_item);
                }
            }
        }
    }

    inspection_count.sort();
    inspection_count.into_iter().rev().take(2).product()
}

fn parse_operation<'a>(input: &'a str) -> IResult<&'a str, Box<dyn Fn(usize) -> usize + 'a>> {
    let mut parser = map(
        tuple((
            tag("old"),
            alt((tag(" + "), tag(" - "), tag(" * "))),
            alt((tag("old"), digit1)),
        )),
        |(_old, operator, rhs)| -> Box<dyn Fn(usize) -> usize> {
            match operator {
                " + " => match rhs {
                    "old" => Box::new(|val: usize| 2 * val),
                    digits => Box::new(|val: usize| val + digits.parse::<usize>().unwrap()),
                },
                " - " => match rhs {
                    "old" => Box::new(|_val: usize| 0),
                    digits => Box::new(|val: usize| val - digits.parse::<usize>().unwrap()),
                },
                " * " => match rhs {
                    "old" => Box::new(|val: usize| val * val),
                    digits => Box::new(|val: usize| val * digits.parse::<usize>().unwrap()),
                },
                _ => panic!("Encountered unexpected operator"),
            }
        },
    );

    parser(input)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        let operation = |x: usize| x + 3;
        let parsed_operation = parse_operation("old + 3").unwrap().1;

        assert_eq!(parsed_operation(2), operation(2));

        let operation = |x: usize| x * x;
        let parsed_operation = parse_operation("old * old").unwrap().1;

        assert_eq!(parsed_operation(5), operation(5));
    }

    #[test]
    fn test_part_one() {
        let data = parse_data("data/11-example.txt");
        assert_eq!(part_one(&data), 10605);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/11-example.txt");
        assert_eq!(part_two(&data), 2713310158);
    }
}
