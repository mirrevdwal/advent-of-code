use itertools::Itertools;
use std::fs::read_to_string;
use std::hash::Hash;

fn main() {
    let data = parse_data("data/06.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    get_unique_sequence_index(&get_characters(data), 4)
}

fn part_two(data: &str) -> usize {
    get_unique_sequence_index(&get_characters(data), 14)
}

fn get_unique_sequence_index<T>(sequence: &[T], length: usize) -> usize
where
    T: Hash + Eq,
{
    let window_index = sequence
        .windows(length)
        .position(|window| window.iter().unique().count() == length)
        .expect("Could not find starting sequence");

    window_index + length
}

fn get_characters(data: &str) -> Vec<char> {
    data
        .lines()
        .next()
        .expect("Datafile was emtpy")
        .chars()
        .collect::<Vec<_>>()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/06-example1.txt");
        assert_eq!(7, part_one(&data));

        let data = parse_data("data/06-example2.txt");
        assert_eq!(5, part_one(&data));

        let data = parse_data("data/06-example3.txt");
        assert_eq!(6, part_one(&data));

        let data = parse_data("data/06-example4.txt");
        assert_eq!(10, part_one(&data));

        let data = parse_data("data/06-example5.txt");
        assert_eq!(11, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/06-example1.txt");
        assert_eq!(19, part_two(&data));

        let data = parse_data("data/06-example2.txt");
        assert_eq!(23, part_two(&data));

        let data = parse_data("data/06-example3.txt");
        assert_eq!(23, part_two(&data));

        let data = parse_data("data/06-example4.txt");
        assert_eq!(29, part_two(&data));

        let data = parse_data("data/06-example5.txt");
        assert_eq!(26, part_two(&data));
    }
}
