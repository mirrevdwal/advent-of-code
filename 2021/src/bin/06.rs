use std::fs::read_to_string;

const MAX_AGE: usize = 8;

fn main() {
    let data = parse_data("data/06.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let mut lanternfish = get_lanternfish(data);

    for _day in 0..=80 {
        lanternfish = shift_day(lanternfish);
    }

    lanternfish.into_iter().sum::<usize>()
}

fn part_two(data: &str) -> usize {
    let mut lanternfish = get_lanternfish(data);

    for _day in 0..=256 {
        lanternfish = shift_day(lanternfish);
    }

    lanternfish.into_iter().sum::<usize>()
}

fn shift_day(mut lanternfish: [usize; MAX_AGE + 1]) -> [usize; MAX_AGE + 1] {
    let new_lanternfish = lanternfish[0];
    for age in 0..(MAX_AGE) {
        lanternfish[age] = lanternfish[age + 1];
    }
    lanternfish[MAX_AGE] = new_lanternfish;
    lanternfish[MAX_AGE - 2] += new_lanternfish;

    lanternfish
}

fn get_lanternfish(input: &str) -> [usize; MAX_AGE + 1] {
    input
        .trim()
        .split(',')
        .map(|age_str| age_str.parse::<usize>().unwrap())
        .fold([0; MAX_AGE + 1], |mut acc, age| {
            acc[age + 1] += 1;
            acc
        })
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/06-example.txt");
        assert_eq!(5934, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/06-example.txt");
        assert_eq!(26984457539, part_two(&data));
    }
}
