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
    let mut lines = data.lines();
    let mut max = 0;

    loop {
        let elf = lines.by_ref().take_while(|line| !line.is_empty());

        let total = elf
            .filter_map(|line| line.parse::<usize>().ok())
            .sum::<usize>();

        max = usize::max(max, total);

        if total == 0 {
            break;
        }
    }

    max
}

fn part_two(data: &str) -> usize {
    let mut lines = data.lines();
    let mut elves = Vec::new();

    loop {
        let elf = lines.by_ref().take_while(|line| !line.is_empty());

        let total = elf
            .filter_map(|line| line.parse::<usize>().ok())
            .sum::<usize>();

        elves.push(total);

        if total == 0 {
            break;
        }
    }

    elves.sort();
    elves.iter().rev().take(3).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/01-example.txt");
        assert_eq!(24000, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/01-example.txt");
        assert_eq!(45000, part_two(&data));
    }
}
