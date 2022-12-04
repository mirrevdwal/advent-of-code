use std::fs::read_to_string;

type Range = Vec<usize>;

fn main() {
    let data = parse_data("data/04.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    data.lines()
        .filter_map(parse_line)
        .map(|(range1, range2)| {
	    (range_to_mask(range1), range_to_mask(range2))
        }).filter(|(mask1, mask2)| {
	    let intersection = mask1 & mask2;
	    intersection == *mask1 || intersection == *mask2
	})
        .count()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .filter_map(parse_line)
        .map(|(range1, range2)| {
	    (range_to_mask(range1), range_to_mask(range2))
        }).filter(|(mask1, mask2)| {
	    let intersection = mask1 & mask2;
	    intersection != 0
	})
        .count()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn parse_line(line: &str) -> Option<(Range, Range)> {
    let pair = line.split_once(',').unwrap();
    Some((parse_range(pair.0).ok()?, parse_range(pair.1).ok()?))
}

fn parse_range(range_str: &str) -> Result<Range, Box<dyn std::error::Error>> {
    let (lower_bound, upper_bound) = range_str.split_once('-').expect("Could not find range");
    Ok((lower_bound.parse::<usize>()?..=upper_bound.parse::<usize>()?).collect::<Range>())
}

fn range_to_mask(range: Range) -> u128 {
    range.iter().fold(0, |mask, num| mask | (1 << num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/04-example.txt");
        assert_eq!(2, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/04-example.txt");
        assert_eq!(4, part_two(&data));
    }
}
