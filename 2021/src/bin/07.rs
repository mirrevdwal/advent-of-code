use std::fs::read_to_string;

fn main() {
    let data = parse_data("data/07.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let crabs = get_positions(data).collect::<Vec<_>>();
    let min = *crabs.iter().min().expect("Could not find min position");
    let max = *crabs.iter().max().expect("Could not find max position");

    (min..=max)
        .map(|position| {
            crabs
                .iter()
                .map(|crab| isize::abs(crab - position) as usize)
                .sum()
        })
        .min()
        .expect("Could not find optimal position")
}

fn part_two(data: &str) -> usize {
    let crabs = get_positions(data).collect::<Vec<_>>();
    let min = *crabs.iter().min().expect("Could not find min position");
    let max = *crabs.iter().max().expect("Could not find max position");

    (min..=max)
        .map(|position| {
            crabs
                .iter()
                .map(|crab| {
                    let distance = isize::abs(crab - position) as usize;
                    (distance * (distance + 1)) / 2
                })
                .sum()
        })
        .min()
        .expect("Could not find optimal position")
}

fn get_positions(input: &str) -> impl Iterator<Item = isize> + '_ {
    input
        .lines()
        .next()
        .expect("Datafile was emtpy")
        .split(',')
        .map(|position| position.parse().unwrap())
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/07-example.txt");
        assert_eq!(part_one(&data), 37);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/07-example.txt");
        assert_eq!(part_two(&data), 168);
    }
}
