use std::{collections::HashMap, fs::read_to_string};

type Register = char;
type Offset = isize;

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset),
}

#[derive(Debug)]
struct Runtime {
    variables: HashMap<char, usize>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn set_register(&mut self, register: char, value: usize) {
	self.variables.insert(register, value);
    }

    fn run_instructions(
        &mut self,
        instructions: &[Instruction],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut index = 0;

        loop {
            let instruction = &instructions[index];

            match instruction {
                Instruction::Half(register) => {
                    *self.variables.entry(*register).or_insert(0) /= 2;
                    index += 1;
                }
                Instruction::Triple(register) => {
                    *self.variables.entry(*register).or_insert(0) *= 3;
                    index += 1;
                }
                Instruction::Increment(register) => {
                    *self.variables.entry(*register).or_insert(0) += 1;
                    index += 1;
                }
                Instruction::Jump(offset) => {
                    index = (index as isize + offset) as usize;
                }
                Instruction::JumpIfEven(register, offset) => {
                    if *self.variables.entry(*register).or_insert(0) % 2 == 0 {
                        index = (index as isize + offset) as usize;
                    } else {
                        index += 1;
                    }
                }
                Instruction::JumpIfOne(register, offset) => {
                    if *self.variables.entry(*register).or_insert(0) == 1 {
                        index = (index as isize + offset) as usize;
                    } else {
                        index += 1;
                    }
                }
            }

            if index >= instructions.len() {
                return Ok(());
            }
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut segments = value.split(" ");

        let instruction = segments.next().ok_or("Could not find instruction")?;

        match instruction {
            "hlf" => {
                let register = segments
                    .next()
                    .ok_or("Could not find register for hlf instruction")?
                    .parse::<char>()
                    .map_err(|_| "Could not parse register as char")?;
                return Ok(Instruction::Half(register));
            }
            "tpl" => {
                let register = segments
                    .next()
                    .ok_or("Could not find register for tpl instruction")?
                    .parse::<char>()
                    .map_err(|_| "Could not parse register as char")?;
                return Ok(Instruction::Triple(register));
            }
            "inc" => {
                let register = segments
                    .next()
                    .ok_or("Could not find register for inc instruction")?
                    .parse::<char>()
                    .map_err(|_| "Could not parse register as char")?;
                return Ok(Instruction::Increment(register));
            }
            "jmp" => {
                let offset = segments
                    .next()
                    .ok_or("Could not find offset for jmp instruction")?
                    .parse::<isize>()
                    .map_err(|_| "Could not parse offset as isize")?;
                return Ok(Instruction::Jump(offset));
            }
            "jie" => {
                let register = segments
                    .next()
                    .ok_or("Could not find register for jie instruction")?
                    .chars()
                    .next()
                    .ok_or("Empty register in jie instruction")?;
                let offset = segments
                    .next()
                    .ok_or("Cound not find offset for jie instruction")?
                    .parse::<isize>()
                    .map_err(|_| "Could not parse register as isize")?;
                return Ok(Instruction::JumpIfEven(register, offset));
            }
            "jio" => {
                let register = segments
                    .next()
                    .ok_or("Could not find register for jio instruction")?
                    .chars()
                    .next()
                    .ok_or("Empty register in jio instruction")?;
                let offset = segments
                    .next()
                    .ok_or("Cound not find offset for jio instruction")?
                    .parse::<isize>()
                    .map_err(|_| "Could not parse register as isize")?;
                return Ok(Instruction::JumpIfOne(register, offset));
            }
            _ => return Err("Unexpected instruction"),
        }
    }
}

fn main() {
    let data = read_to_string("data/23.txt").expect("Could not read datafile");

    let instructions: Vec<Instruction> = data
        .lines()
        .filter_map(|line| line.try_into().ok())
        .collect::<Vec<_>>();

    print!("Part 1: ");
    let mut runtime = Runtime::new();
    match runtime.run_instructions(&instructions) {
        Ok(_) => println!("{:?}", runtime.variables),
        Err(e) => println!("Error while running instructions: {:?}", e),
    }

    print!("Part 2: ");
    let mut runtime = Runtime::new();
    runtime.set_register('a', 1);
    match runtime.run_instructions(&instructions) {
        Ok(_) => println!("{:?}", runtime.variables),
        Err(e) => println!("Error while running instructions: {:?}", e),
    }
}
