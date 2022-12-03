use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let data = parse_data("data/03.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let (first, second) = get_compartments(line);
            *first
                .intersection(&second)
                .next()
                .expect("Could not find overlap between compartments")
        })
        .map(get_priority)
        .sum::<usize>()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|left, right| left.intersection(&right).copied().collect::<HashSet<_>>())
                .expect("Could not find overlap between elves")
                .into_iter()
                .next()
                .expect("Empty overlap between elves")
        })
        .map(get_priority)
        .sum::<usize>()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn get_compartments(line: &str) -> (HashSet<char>, HashSet<char>) {
    let num_items = line.len();
    let mut characters = line.chars();

    let first = characters
        .by_ref()
        .take(num_items / 2)
        .collect::<HashSet<_>>();
    let second = characters.collect::<HashSet<_>>();

    (first, second)
}

fn get_priority(character: char) -> usize {
    (if character.is_uppercase() {
        character as u8 - b'A' + 27
    } else {
        character as u8 - b'a' + 1
    }) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/03-example.txt");
        assert_eq!(157, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/03-example.txt");
        assert_eq!(70, part_two(&data));
    }
}
