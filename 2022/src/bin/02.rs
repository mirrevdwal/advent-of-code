use std::fs::read_to_string;

#[derive(Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Shape {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err("Invalid shape"),
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .next()
            .ok_or("Tried to parse empty &str as Shape")?
            .try_into()
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.get_score() as isize - other.get_score() as isize).rem_euclid(3) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            2 => std::cmp::Ordering::Less,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Shape {
    fn get_score(&self) -> usize {
        match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn get_shape(index: usize) -> Self {
        match index % 3 {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for Instruction {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Instruction::Lose),
            'Y' => Ok(Instruction::Draw),
            'Z' => Ok(Instruction::Win),
            _ => Err("Invalid shape"),
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .chars()
            .next()
            .ok_or("Tried to parse empty &str as Instruction")?
            .try_into()
    }
}

impl Instruction {
    fn get_offset(&self) -> usize {
        match &self {
            Instruction::Lose => 2,
            Instruction::Draw => 0,
            Instruction::Win => 1,
        }
    }

    fn get_score(&self) -> usize {
        match &self {
            Instruction::Lose => 0,
            Instruction::Draw => 3,
            Instruction::Win => 6,
        }
    }
}

fn main() {
    let data = parse_data("data/02.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    data.lines()
        .filter_map(|line| get_match(line).ok())
        .map(|(opponent, player)| player.get_score() + get_match_score(opponent, player))
        .sum::<usize>()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .filter_map(|line| get_instruction(line).ok())
        .map(|(opponent, instruction)| {
            let player = Shape::get_shape(opponent.get_score() - 1 + instruction.get_offset());
            player.get_score() + instruction.get_score()
        })
        .sum::<usize>()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn get_match(line: &str) -> Result<(Shape, Shape), Box<dyn std::error::Error>> {
    let shapes = line.split_once(' ').unwrap();
    Ok((shapes.0.try_into()?, shapes.1.try_into()?))
}

fn get_match_score(opponent_shape: Shape, player_shape: Shape) -> usize {
    match player_shape.cmp(&opponent_shape) {
        std::cmp::Ordering::Less => 0,
        std::cmp::Ordering::Equal => 3,
        std::cmp::Ordering::Greater => 6,
    }
}

fn get_instruction(line: &str) -> Result<(Shape, Instruction), Box<dyn std::error::Error>> {
    let shapes = line.split_once(' ').unwrap();
    Ok((shapes.0.try_into()?, shapes.1.try_into()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/02-example.txt");
        assert_eq!(15, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/02-example.txt");
        assert_eq!(12, part_two(&data));
    }
}
