use std::{collections::HashSet, fs::read_to_string};

type Coord = (isize, isize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(&self) -> Coord {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Instruction {
    direction: Direction,
    amount: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let direction = match parts
            .next()
            .ok_or("Could not find direction in instruction")?
        {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unexpected direction in instruction"),
        };

        let amount = match parts
            .next()
            .ok_or("Could not find number of steps in instruction")?
        {
            num if num.parse::<usize>().is_ok() => num.parse::<usize>().unwrap(),
            _ => panic!("Unexpected second segment in instruction"),
        };

        Ok(Instruction { direction, amount })
    }
}

fn main() {
    let data = parse_data("data/09.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let rope_size = 2;

    let instructions = data
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .rev()
        .collect::<Vec<_>>();

    let positions = parse_instructions(instructions, rope_size);

    let unique_tail_positions = positions
        .last()
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<Coord>>();

    unique_tail_positions.len()
}

fn part_two(data: &str) -> usize {
    let rope_size = 10;

    let instructions = data
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .rev()
        .collect::<Vec<_>>();

    let positions = parse_instructions(instructions, rope_size);

    let unique_tail_positions = positions
        .last()
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<Coord>>();

    unique_tail_positions.len()
}

fn parse_instructions(mut instructions: Vec<Instruction>, rope_size: usize) -> Vec<Vec<Coord>> {
    let mut rope_positions = vec![vec![(0, 0)]; rope_size];

    while let Some(instruction) = instructions.pop() {
        let offset = instruction.direction.to_offset();

        // Move head
        let mut new_head_coord = *rope_positions[0].last().unwrap();
        new_head_coord.0 += offset.0;
        new_head_coord.1 += offset.1;
        rope_positions[0].push(new_head_coord);

        // Check whether tail needs to move
        for knot_positions in rope_positions.iter_mut().take(rope_size).skip(1) {
            let mut new_tail_coord = *knot_positions.last().unwrap();
            let (x_off, y_off) = get_offset(new_tail_coord, new_head_coord);
            if x_off.abs() > 1 || y_off.abs() > 1 {
                // Check if row or column is the same for head and tail
                // or (part 2) a diagonally adjacent knot has moved away diagonally
                if (x_off == 0 || y_off == 0) || (x_off.abs() > 1 && y_off.abs() > 1) {
                    new_tail_coord.0 += x_off / 2;
                    new_tail_coord.1 += y_off / 2;
                } else if x_off.abs() > 1 {
                    new_tail_coord.0 += x_off / 2;
                    new_tail_coord.1 += y_off;
                } else {
                    new_tail_coord.0 += x_off;
                    new_tail_coord.1 += y_off / 2;
                }
                knot_positions.push(new_tail_coord);
            }

            // Current tail will be head for next item in rope
            new_head_coord = new_tail_coord;
        }

        // Add instruction to queue if unfinished
        if instruction.amount > 1 {
            instructions.push(Instruction {
                direction: instruction.direction,
                amount: instruction.amount - 1,
            });
        }
    }

    rope_positions
}

fn get_offset(coord1: Coord, coord2: Coord) -> Coord {
    (coord2.0 - coord1.0, coord2.1 - coord1.1)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/09-example1.txt");
        assert_eq!(part_one(&data), 13);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/09-example1.txt");
        assert_eq!(part_two(&data), 1);
    }

    #[test]
    fn test_part_two_large_example() {
        let data = parse_data("data/09-example2.txt");
        assert_eq!(part_two(&data), 36);
    }
}
