use std::fs::read_to_string;

fn main() {
    let data = parse_data("data/01.txt");

    let answer_one = part_one(&data);
    println!("Part 1: {answer_one}");

    let answer_two = part_two(&data);
    println!("Part 2: {answer_two}");
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn part_one(data: &str) -> usize {
    let lines = data.lines();

    lines.map(|line| {
        let mut numbers = line.chars().filter_map(|char| char.to_digit(10));
        let first = numbers.next().expect("No digit found in line");
        first * 10 + if let Some(last) = numbers.last() { last } else { first }
    }).sum::<u32>() as usize
}

fn part_two(data: &str) -> usize {
    let text_numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    data.lines().map(|line| {
        let mut new_line = line.to_owned();
        for (i, text_number) in text_numbers.into_iter().enumerate() {
            let number = (i + 1).to_string();
            new_line = new_line.replace(text_number, &format!("{text_number}{number}{text_number}"));
        }
        let mut numbers = new_line.chars().filter_map(|char| char.to_digit(10));
        let first = numbers.next().expect("No digit found in line");
        first * 10 + if let Some(last) = numbers.last() { last } else { first }
    }).sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/01-example1.txt");
        assert_eq!(142, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/01-example2.txt");
        assert_eq!(281, part_two(&data));
    }
}
