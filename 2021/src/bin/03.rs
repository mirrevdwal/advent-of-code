use std::fs::read_to_string;

fn main() {
    let data = parse_data("data/03.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let line_len = data.lines().next().unwrap().len();
    let char_sum = data
        .lines()
        .map(|line| line.chars().map(get_bit).collect::<Vec<_>>())
        .fold(vec![0; line_len], |acc, cur| {
            (0..line_len).map(|index| acc[index] + cur[index]).collect()
        });

    let gamma_rate = char_sum.iter().map(|x| *x >= 0).collect::<Vec<_>>();
    let epsilon_rate = gamma_rate.iter().map(|x| !x).collect::<Vec<_>>();

    get_decimal(gamma_rate) * get_decimal(epsilon_rate)
}

fn part_two(data: &str) -> usize {
    let mut ox_gen_options = data.lines().collect::<Vec<_>>();

    let mut i = 0;
    while ox_gen_options.len() > 1 {
        let total = ox_gen_options
            .iter()
            .map(|&option| get_bit(option.chars().nth(i).expect("No more characters found")))
            .sum::<isize>();
        ox_gen_options = ox_gen_options
            .into_iter()
            .filter(|option| {
                let character = option.chars().nth(i).unwrap();
                character == if total >= 0 { '1' } else { '0' }
            })
            .collect::<Vec<_>>();
        i += 1;
    }

    let mut co2_scrub_options = data.lines().collect::<Vec<_>>();

    let mut i = 0;
    while co2_scrub_options.len() > 1 {
        let total = co2_scrub_options
            .iter()
            .map(|&option| get_bit(option.chars().nth(i).expect("No more characters found")))
            .sum::<isize>();
        co2_scrub_options = co2_scrub_options
            .into_iter()
            .filter(|option| {
                let character = option.chars().nth(i).unwrap();
                character == if total >= 0 { '0' } else { '1' }
            })
            .collect::<Vec<_>>();
        i += 1;
    }

    let ox_gen = ox_gen_options[0]
        .chars()
        .map(|character| character == '1')
        .collect::<Vec<_>>();
    let co2_scrub = co2_scrub_options[0]
        .chars()
        .map(|character| character == '1')
        .collect::<Vec<_>>();

    get_decimal(ox_gen) * get_decimal(co2_scrub)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn get_bit(character: char) -> isize {
    match character {
        '0' => -1,
        '1' => 1,
        _ => panic!("Encountered unexpected bit"),
    }
}

fn get_decimal(bits: Vec<bool>) -> usize {
    let mut total = 0;
    for bit in bits {
        total *= 2;
        total += usize::from(bit);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/03-example.txt");
        assert_eq!(198, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/03-example.txt");
        assert_eq!(230, part_two(&data));
    }
}
