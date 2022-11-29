use std::collections::{HashMap, LinkedList};

type Num = u16;
type Var = String;

#[derive(PartialEq, Debug)]
enum Value {
    Variable(Var),
    Number(Num),
}

#[derive(PartialEq)]
enum Token {
    Value(Value),
    Instruction(InstructionType),
}

#[derive(PartialEq)]
enum InstructionType {
    Not,
    And,
    Or,
    LShift,
    RShift,
}

#[derive(Debug)]
enum Instruction {
    Set(Value),
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Num),
    RShift(Value, Num),
}

fn main() {
    let data = std::fs::read_to_string("data/07.txt").expect("Could not read datafile");
    let instructions = parse_instructions(data);

    let first_instructions = instructions
        .iter()
        .map(|(x, y)| (x, y))
        .collect::<LinkedList<_>>();
    let variables = read_instructions(first_instructions);

    let a = "a".to_string();

    let answer = variables
        .get(&a)
        .expect("Wanted variable could not be found");

    println!("Part 1: {:#?}", answer);

    let mut second_instructions = instructions
        .iter()
        .filter_map(|(x, y)| if x != "b" { Some((x, y)) } else { None })
        .collect::<LinkedList<_>>();

    let b_instruction = ("b".to_string(), Instruction::Set(Value::Number(*answer)));
    second_instructions.push_front((&b_instruction.0, &b_instruction.1));
    let variables = read_instructions(second_instructions);

    let answer = variables
        .get(&a)
        .expect("Wanted variable could not be found");

    println!("Part 2: {:#?}", answer);
}

fn read_instructions<'a>(
    mut instructions: LinkedList<(&'a Var, &Instruction)>,
) -> HashMap<&'a Var, Num> {
    let mut variables: HashMap<&Var, Num> = HashMap::new();
    while let Some((destination, instruction)) = instructions.pop_front() {
        match &instruction {
            Instruction::Set(value) => {
                let num = match value {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                if let Some(number) = num {
                    variables.insert(destination, *number);
                } else {
                    instructions.push_back((destination, instruction));
                }
            }
            Instruction::Not(value) => {
                let num = match value {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                if let Some(number) = num {
                    variables.insert(destination, !number);
                } else {
                    instructions.push_back((destination, instruction));
                }
            }
            Instruction::And(value1, value2) => {
                let num1 = match value1 {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                let num2 = match value2 {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                if let (Some(number1), Some(number2)) = (num1, num2) {
                    variables.insert(destination, number1 & number2);
                } else {
                    instructions.push_back((destination, instruction));
                }
            }
            Instruction::Or(value1, value2) => {
                let num1 = match value1 {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                let num2 = match value2 {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                if let (Some(number1), Some(number2)) = (num1, num2) {
                    variables.insert(destination, number1 | number2);
                } else {
                    instructions.push_back((destination, instruction));
                }
            }
            Instruction::LShift(value, shift_num) => {
                let num = match value {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                if let Some(number) = num {
                    variables.insert(destination, number << shift_num);
                } else {
                    instructions.push_back((destination, instruction));
                }
            }
            Instruction::RShift(value, shift_num) => {
                let num = match value {
                    Value::Variable(variable) => variables.get(variable),
                    Value::Number(number) => Some(number),
                };
                if let Some(number) = num {
                    variables.insert(destination, number >> shift_num);
                } else {
                    instructions.push_back((destination, instruction));
                }
            }
        }
    }

    variables
}

fn parse_instructions(data: String) -> Vec<(Var, Instruction)> {
    data.lines()
        .map(|line| {
            let mut parts = line.split(" -> ");

            let input = parts.next().expect("Could not find input of instruction");
            let output = parts.next().expect("Could not find output of instruction");

            let mut tokens = input.split_whitespace().map(|word| match word {
                "NOT" => Token::Instruction(InstructionType::Not),
                "AND" => Token::Instruction(InstructionType::And),
                "OR" => Token::Instruction(InstructionType::Or),
                "LSHIFT" => Token::Instruction(InstructionType::LShift),
                "RSHIFT" => Token::Instruction(InstructionType::RShift),
                _ => match word.parse::<u16>() {
                    Ok(number) => Token::Value(Value::Number(number)),
                    Err(_) => Token::Value(Value::Variable(word.to_owned())),
                },
            });

            let first_token = tokens
                .next()
                .expect("Could not find first token of instruction");

            let instruction = match first_token {
                Token::Instruction(instruction) => {
                    if instruction == InstructionType::Not {
                        let second_token = tokens
                            .next()
                            .expect("Could not find variable of NOT instruction");
                        match second_token {
                            Token::Value(value) => Instruction::Not(value),
                            _ => panic!("NOT instruction got unexpected variable"),
                        }
                    } else {
                        panic!("Instruction started with invalid type")
                    }
                }
                Token::Value(value1) => {
                    let second_token = tokens.next();
                    if let Some(instruction) = second_token {
                        if let Token::Instruction(instruction_type) = instruction {
                            match instruction_type {
                                InstructionType::Not => {
                                    panic!("NOT instruction at unexpected position")
                                }
                                InstructionType::And => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after AND instruction");
                                    if let Token::Value(value2) = third_token {
                                        Instruction::And(value1, value2)
                                    } else {
                                        panic!("Found unexpected token after AND instruction")
                                    }
                                }
                                InstructionType::Or => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after OR instruction");
                                    if let Token::Value(value2) = third_token {
                                        Instruction::Or(value1, value2)
                                    } else {
                                        panic!("Found unexpected token after OR instruction")
                                    }
                                }
                                InstructionType::LShift => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after LSHIFT instruction");
                                    if let Token::Value(Value::Number(number)) = third_token {
                                        Instruction::LShift(value1, number)
                                    } else {
                                        panic!("Found unexpected token after LSHIFT instruction")
                                    }
                                }
                                InstructionType::RShift => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after RSHIFT instruction");
                                    if let Token::Value(Value::Number(number)) = third_token {
                                        Instruction::RShift(value1, number)
                                    } else {
                                        panic!("Found unexpected token after RSHIFT instruction")
                                    }
                                }
                            }
                        } else {
                            panic!("Could not find instruction after first variable")
                        }
                    } else {
                        Instruction::Set(value1)
                    }
                }
            };

            (output.to_owned(), instruction)
        })
        .collect()
}
