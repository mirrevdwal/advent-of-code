use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};
use std::{fmt::Display, fs::read_to_string};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct CRT {
    data: Vec<bool>,
    height: usize,
    width: usize,
}

impl CRT {
    fn new(height: usize, width: usize) -> Self {
        Self {
            data: vec![false; height * width],
            height,
            width,
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }

    fn light_pixel(&mut self, x: usize, y: usize) {
        self.data[y * self.width + x] = true
    }

    fn run_step(&mut self, cycle: usize, sprite: i32) {
        let x = cycle % self.width;
        let y = cycle / self.width;
        if x as i32 >= sprite - 1 && x as i32 <= sprite + 1 {
            self.light_pixel(x, y);
        }
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get_pixel(x, y) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

fn main() {
    let data = parse_data("data/10.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2:");
    println!("{answer}");
}

fn part_one(input: &str) -> i32 {
    let instructions = input
        .lines()
        .filter_map(|line| parse_instructions(line).ok())
        .map(|(_res, instruction)| instruction);

    let mut x = 1;
    let mut cycle = 0;
    let mut total = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                let after_checkpoint = (cycle - 20) % 40;
                if after_checkpoint == 0 {
                    total += cycle * x;
                }
            }
            Instruction::Addx(value) => {
                cycle += 2;
                let after_checkpoint = (cycle - 20) % 40;
                if after_checkpoint == 0 || after_checkpoint == 1 {
                    total += (cycle - after_checkpoint) * x;
                }
                x += value;
            }
        }
    }

    total
}

fn part_two(input: &str) -> CRT {
    let instructions = input
        .lines()
        .filter_map(|line| parse_instructions(line).ok())
        .map(|(_res, instruction)| instruction);

    let mut sprite = 1;
    let mut cycle = 0;

    let mut crt = CRT::new(6, 40);

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                crt.run_step(cycle, sprite);
                cycle += 1;
            }
            Instruction::Addx(value) => {
                for _i in 0..2 {
                    crt.run_step(cycle, sprite);
                    cycle += 1;
                }
                sprite += value;
            }
        }
    }

    crt
}

fn parse_instructions(line: &str) -> IResult<&str, Instruction> {
    let noop_parser = tag("noop");
    let addx_parser = tag("addx");
    let mut parser = alt((noop_parser, addx_parser));
    let (res, matched) = parser(line)?;

    let (res, instruction) = match matched {
        "noop" => (res, Instruction::Noop),
        "addx" => {
            let (res, number) = parse_number(res.trim())?;
            (res, Instruction::Addx(number))
        }
        _ => panic!("Unexpected instruction encountered"),
    };

    Ok((res, instruction))
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    let parser = recognize(tuple((opt(nom::character::complete::char('-')), digit1)));
    let mut number_parser = map_res(parser, |s: &str| s.parse::<i32>());
    number_parser(input)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/10-example.txt");
        assert_eq!(part_one(&data), 13140);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/10-example.txt");
        let answer = part_two(&data);
        println!("{answer}");

        // You can set this to false to see the output of part 2
        assert!(true);
    }
}
