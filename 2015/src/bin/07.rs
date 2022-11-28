use std::collections::{HashMap, LinkedList};

type Num = u16;
type Var = String;

#[derive(PartialEq, Debug)]
enum Value {
    VAR(Var),
    NUM(Num),
}

#[derive(PartialEq)]
enum Token {
    VALUE(Value),
    INSTRUCTION(InstructionType),
}

#[derive(PartialEq)]
enum InstructionType {
    NOT,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

#[derive(Debug)]
enum Instruction {
    SET(Value),
    NOT(Value),
    AND(Value, Value),
    OR(Value, Value),
    LSHIFT(Value, Num),
    RSHIFT(Value, Num),
}

fn main() {
    let data = std::fs::read_to_string("data/07.txt").expect("Could not read datafile");
    let instructions = parse_instructions(data);

    let first_instructions = instructions.iter().map(|(x,y)| (x,y)).collect::<LinkedList<_>>();
    let variables = read_instructions(first_instructions);

    let a = "a".to_string();

    let answer = variables
        .get(&a)
        .expect("Wanted variable could not be found");

    println!("Part 1: {:#?}", answer);

    let mut second_instructions = instructions.iter().filter_map(|(x,y)| {
	if x != "b" {
	    Some((x,y))
	} else {
	    None
	}
    }).collect::<LinkedList<_>>();

    let b_instruction = ("b".to_string(), Instruction::SET(Value::NUM(*answer)));
    second_instructions.push_front((&b_instruction.0, &b_instruction.1));
    let variables = read_instructions(second_instructions);

    let answer = variables
        .get(&a)
        .expect("Wanted variable could not be found");

    println!("Part 2: {:#?}", answer);
}

fn read_instructions<'a>(mut instructions: LinkedList<(&'a Var, &Instruction)>) -> HashMap<&'a Var, Num> {
    let mut variables: HashMap<&Var, Num> = HashMap::new();
    loop {
        if let Some((destination, instruction)) = instructions.pop_front() {
            match &instruction {
                Instruction::SET(value) => {
                    let num = match value {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    if let Some(number) = num {
                        variables.insert(destination, *number);
                    } else {
                        instructions.push_back((destination, instruction));
                    }
                }
                Instruction::NOT(value) => {
                    let num = match value {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    if let Some(number) = num {
                        variables.insert(destination, !number);
                    } else {
                        instructions.push_back((destination, instruction));
                    }
                }
                Instruction::AND(value1, value2) => {
                    let num1 = match value1 {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    let num2 = match value2 {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    if let (Some(number1), Some(number2)) = (num1, num2) {
                        variables.insert(destination, number1 & number2);
                    } else {
                        instructions.push_back((destination, instruction));
                    }
                }
                Instruction::OR(value1, value2) => {
                    let num1 = match value1 {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    let num2 = match value2 {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    if let (Some(number1), Some(number2)) = (num1, num2) {
                        variables.insert(destination, number1 | number2);
                    } else {
                        instructions.push_back((destination, instruction));
                    }
                }
                Instruction::LSHIFT(value, shift_num) => {
                    let num = match value {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    if let Some(number) = num {
                        variables.insert(destination, number << shift_num);
                    } else {
                        instructions.push_back((destination, instruction));
                    }
                }
                Instruction::RSHIFT(value, shift_num) => {
                    let num = match value {
                        Value::VAR(variable) => variables.get(variable),
                        Value::NUM(number) => Some(number),
                    };
                    if let Some(number) = num {
                        variables.insert(destination, number >> shift_num);
                    } else {
                        instructions.push_back((destination, instruction));
                    }
                }
            }
        } else {
            break;
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

            let mut tokens = input.split(" ").map(|word| match word {
                "NOT" => Token::INSTRUCTION(InstructionType::NOT),
                "AND" => Token::INSTRUCTION(InstructionType::AND),
                "OR" => Token::INSTRUCTION(InstructionType::OR),
                "LSHIFT" => Token::INSTRUCTION(InstructionType::LSHIFT),
                "RSHIFT" => Token::INSTRUCTION(InstructionType::RSHIFT),
                _ => match word.parse::<u16>() {
                    Ok(number) => Token::VALUE(Value::NUM(number)),
                    Err(_) => Token::VALUE(Value::VAR(word.to_owned())),
                },
            });

            let first_token = tokens
                .next()
                .expect("Could not find first token of instruction");

            let instruction = match first_token {
                Token::INSTRUCTION(instruction) => {
                    if instruction == InstructionType::NOT {
                        let second_token = tokens
                            .next()
                            .expect("Could not find variable of NOT instruction");
                        match second_token {
                            Token::VALUE(value) => Instruction::NOT(value),
                            _ => panic!("NOT instruction got unexpected variable"),
                        }
                    } else {
                        panic!("Instruction started with invalid type")
                    }
                }
                Token::VALUE(value1) => {
                    let second_token = tokens.next();
                    if let Some(instruction) = second_token {
                        if let Token::INSTRUCTION(instruction_type) = instruction {
                            match instruction_type {
                                InstructionType::NOT => {
                                    panic!("NOT instruction at unexpected position")
                                }
                                InstructionType::AND => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after AND instruction");
                                    if let Token::VALUE(value2) = third_token {
                                        Instruction::AND(value1, value2)
                                    } else {
                                        panic!("Found unexpected token after AND instruction")
                                    }
                                }
                                InstructionType::OR => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after OR instruction");
                                    if let Token::VALUE(value2) = third_token {
                                        Instruction::OR(value1, value2)
                                    } else {
                                        panic!("Found unexpected token after OR instruction")
                                    }
                                }
                                InstructionType::LSHIFT => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after LSHIFT instruction");
                                    if let Token::VALUE(Value::NUM(number)) = third_token {
                                        Instruction::LSHIFT(value1, number)
                                    } else {
                                        panic!("Found unexpected token after LSHIFT instruction")
                                    }
                                }
                                InstructionType::RSHIFT => {
                                    let third_token = tokens
                                        .next()
                                        .expect("Could not find token after RSHIFT instruction");
                                    if let Token::VALUE(Value::NUM(number)) = third_token {
                                        Instruction::RSHIFT(value1, number)
                                    } else {
                                        panic!("Found unexpected token after RSHIFT instruction")
                                    }
                                }
                            }
                        } else {
                            panic!("Could not find instruction after first variable")
                        }
                    } else {
                        Instruction::SET(value1)
                    }
                }
            };

            (output.to_owned(), instruction)
        })
        .collect()
}
