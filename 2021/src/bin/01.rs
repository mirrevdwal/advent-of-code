use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let data = parse_data("data/01.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");
    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    data.lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .tuple_windows()
        .filter(|(previous, current)| previous < current)
        .count()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .tuple_windows::<(_, _, _)>()
        .map(|(first, second, third)| first + second + third)
        .tuple_windows()
        .filter(|(previous, current)| previous < current)
        .count()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/01-example.txt");
        assert_eq!(7, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/01-example.txt");
        assert_eq!(5, part_two(&data));
    }
}
