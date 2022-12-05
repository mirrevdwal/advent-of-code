use itertools::Itertools;
use std::{collections::VecDeque, fs::read_to_string};

type Crate = char;
type Stack = VecDeque<Crate>;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut words = line.split_whitespace();

        let amount = words
            .nth(1)
            .ok_or("Could not find number of crates in line")?
            .parse()?;
        let from = words
            .nth(1)
            .ok_or("Could not find start stack in line")?
            .parse()?;
        let to = words
            .nth(1)
            .ok_or("Could not find end stack in line")?
            .parse()?;

        Ok(Instruction { amount, from, to })
    }
}

fn main() {
    let data = parse_data("data/05.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> String {
    let (mut stacks, instructions) = parse_input(data);

    for instruction in instructions {
        let stack = &mut stacks[instruction.from - 1];
        let moved = stack.split_off(stack.len() - (instruction.amount));
        stacks[instruction.to - 1].extend(moved.into_iter().rev())
    }

    stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop_back())
        .collect()
}

fn part_two(data: &str) -> String {
    let (mut stacks, instructions) = parse_input(data);

    for instruction in instructions {
        let stack = &mut stacks[instruction.from - 1];
        let moved = stack.split_off(stack.len() - (instruction.amount));
        stacks[instruction.to - 1].extend(moved.into_iter())
    }

    stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop_back())
        .collect()
}

fn parse_input(data: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let num_stacks = data.lines().next().unwrap().len() / 3;
    let mut stacks: Vec<Stack> = vec![VecDeque::new(); num_stacks];

    let mut lines = data.lines();

    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let chunk = line.chars().chunks(4);
            chunk
                .into_iter()
                .enumerate()
                .filter_map(|(index, mut chunk)| {
                    let character = chunk.nth(1).unwrap();
                    (character != ' ').then_some((index, character))
                })
                .for_each(|(index, character)| {
                    let stack = stacks.get_mut(index).unwrap();
                    stack.push_front(character);
                });
        });

    // Remove stack number from stack
    stacks.iter_mut().for_each(|stack| {
        let _ = stack.pop_front();
    });

    let instructions = lines
        .filter_map(|line| Instruction::try_from(line).ok())
        .collect::<Vec<_>>();

    (stacks, instructions)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/05-example.txt");
        assert_eq!("CMZ", &part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/05-example.txt");
        assert_eq!("MCD", &part_two(&data));
    }
}
