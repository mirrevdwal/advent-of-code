use std::fs::read_to_string;

#[derive(Default)]
struct Position {
    depth: usize,
    horizontal: usize,
    aim: usize,
}

impl Position {
    fn parse_move(&mut self, movement: Move) {
        match movement {
            Move::Up(amount) => self.depth -= amount,
            Move::Down(amount) => self.depth += amount,
            Move::Forward(amount) => self.horizontal += amount,
        }
    }

    fn parse_complicated_move(&mut self, movement: Move) {
        match movement {
            Move::Up(amount) => self.aim -= amount,
            Move::Down(amount) => self.aim += amount,
            Move::Forward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
        }
    }
}

enum Move {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value
            .split_once(' ')
            .expect("Could not split move string into parts");
        let amount = parts
            .1
            .parse::<usize>()
            .expect("Could not parse amount as usize");
        match parts.0 {
            "up" => Ok(Move::Up(amount)),
            "down" => Ok(Move::Down(amount)),
            "forward" => Ok(Move::Forward(amount)),
            _ => Err("Invalid move direction"),
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
    let mut position = Position::default();
    data.lines()
        .filter_map(|line| line.try_into().ok())
        .for_each(|movement: Move| {
            position.parse_move(movement);
        });
    position.horizontal * position.depth
}

fn part_two(data: &str) -> usize {
    let mut position = Position::default();
    data.lines()
        .filter_map(|line| line.try_into().ok())
        .for_each(|movement: Move| {
            position.parse_complicated_move(movement);
        });
    position.horizontal * position.depth
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/02-example.txt");
        assert_eq!(150, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/02-example.txt");
        assert_eq!(900, part_two(&data));
    }
}
