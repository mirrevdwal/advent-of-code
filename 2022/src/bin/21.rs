use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::{complete, is_alphabetic},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

type Monkey<'a> = (&'a str, Operation<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Operation<'a> {
    Assign(i64),
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
}

impl<'a> Operation<'a> {
    fn calculate(&self, known_data: &HashMap<&str, i64>) -> Option<i64> {
        match self {
            Operation::Assign(value) => Some(*value),
            Operation::Add(lhs, rhs) => Some(known_data.get(lhs)? + known_data.get(rhs)?),
            Operation::Subtract(lhs, rhs) => Some(known_data.get(lhs)? - known_data.get(rhs)?),
            Operation::Multiply(lhs, rhs) => Some(known_data.get(lhs)? * known_data.get(rhs)?),
            Operation::Divide(lhs, rhs) => Some(known_data.get(lhs)? / known_data.get(rhs)?),
        }
    }

    fn inverse(self, destination: &'a str) -> [(&str, Self); 2] {
        match self {
            Operation::Add(lhs, rhs) => [
                (lhs, Operation::Subtract(destination, rhs)),
                (rhs, Operation::Subtract(destination, lhs)),
            ],
            Operation::Subtract(lhs, rhs) => [
                (lhs, Operation::Add(destination, rhs)),
                (rhs, Operation::Subtract(lhs, destination)),
            ],
            Operation::Multiply(lhs, rhs) => [
                (lhs, Operation::Divide(destination, rhs)),
                (rhs, Operation::Divide(destination, lhs)),
            ],
            Operation::Divide(lhs, rhs) => [
                (lhs, Operation::Multiply(destination, rhs)),
                (rhs, Operation::Divide(lhs, destination)),
            ],
            Operation::Assign(_) => panic!("Unexpected instruction to invert"),
        }
    }
}

fn main() {
    let data = read_data("data/21.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> i64 {
    let monkeys = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_res, monkey)| monkey);

    let mut known_monkeys: HashMap<&str, i64> = HashMap::new();
    let mut dependencies: HashMap<&str, Vec<Monkey>> = HashMap::new();

    let (assignments, operations): (Vec<_>, Vec<_>) =
        monkeys.partition(|(_id, operation)| matches!(operation, Operation::Assign(_)));

    for operation in operations {
        match operation.1 {
            Operation::Add(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Multiply(lhs, rhs)
            | Operation::Divide(lhs, rhs) => {
                dependencies.entry(lhs).or_default().push(operation.clone());
                dependencies.entry(rhs).or_default().push(operation);
            }
            Operation::Assign(_) => unreachable!(),
        }
    }

    let mut todo = assignments.into_iter().collect::<VecDeque<_>>();

    while let Some((destination, assign)) = todo.pop_front() {
        match assign {
            Operation::Assign(value) => {
                known_monkeys.insert(destination, value);
                if let Some(solved_dependencies) = dependencies.remove(destination) {
                    for (dep_dest, dep_op) in solved_dependencies {
                        if let Some(value) = dep_op.calculate(&known_monkeys) {
                            todo.push_back((dep_dest, Operation::Assign(value)))
                        }
                    }
                }
            }
            _ => panic!("Unexpected operation in assignment queue"),
        }
    }

    *known_monkeys
        .get("root")
        .expect("Could not find root monkey")
}

fn part_two(data: &str) -> i64 {
    let (root, monkeys): (Vec<_>, Vec<_>) = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_res, monkey)| monkey)
        .partition(|(name, _op)| name == &"root");

    let mut known_monkeys: HashMap<&str, i64> = HashMap::new();
    let mut dependencies: HashMap<&str, Vec<Monkey>> = HashMap::new();

    let (assignments, operations): (Vec<_>, Vec<_>) = monkeys
        .into_iter()
        .partition(|(_id, operation)| matches!(operation, Operation::Assign(_)));

    for operation in operations {
        match operation.1 {
            Operation::Add(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Multiply(lhs, rhs)
            | Operation::Divide(lhs, rhs) => {
                dependencies.entry(lhs).or_default().push(operation.clone());
                dependencies.entry(rhs).or_default().push(operation);
            }
            Operation::Assign(_) => unreachable!(),
        }
    }

    let mut todo = assignments.into_iter().collect::<VecDeque<_>>();

    let humn_position = todo
        .iter()
        .position(|(name, _op)| name == &"humn")
        .expect("Could not find humn");
    todo.remove(humn_position);

    while let Some((destination, assign)) = todo.pop_front() {
        match assign {
            Operation::Assign(value) => {
                known_monkeys.insert(destination, value);
                if let Some(solved_dependencies) = dependencies.remove(destination) {
                    for (dep_dest, dep_op) in solved_dependencies {
                        if let Some(value) = dep_op.calculate(&known_monkeys) {
                            todo.push_back((dep_dest, Operation::Assign(value)))
                        }
                    }
                }
            }
            _ => panic!("Unexpected operation in assignment queue"),
        }
    }

    match root[0].1 {
        Operation::Add(lhs, rhs)
        | Operation::Subtract(lhs, rhs)
        | Operation::Multiply(lhs, rhs)
        | Operation::Divide(lhs, rhs) => match (known_monkeys.get(lhs), known_monkeys.get(rhs)) {
            (None, Some(value)) => {
                todo.push_back((lhs, Operation::Assign(*value)));
            }
            (Some(value), None) => {
                todo.push_back((rhs, Operation::Assign(*value)));
            }
            (None, None) => panic!("Both root variables depend on humn"),
            (Some(_), Some(_)) => panic!("Equation not necessary to solve problem"),
        },
        Operation::Assign(_) => panic!("Expected root to be dependent on two variables!"),
    }

    for (destination, operation) in dependencies.clone().into_values().flatten() {
        let inverses = operation.inverse(destination);
        for inverse in inverses {
            match inverse.1 {
                Operation::Add(lhs, rhs)
                | Operation::Subtract(lhs, rhs)
                | Operation::Multiply(lhs, rhs)
                | Operation::Divide(lhs, rhs) => {
                    dependencies.entry(lhs).or_default().push(inverse.clone());
                    dependencies.entry(rhs).or_default().push(inverse);
                }
                Operation::Assign(_) => unreachable!(),
            }
        }
    }

    while let Some((destination, assign)) = todo.pop_front() {
        match assign {
            Operation::Assign(value) => {
                known_monkeys.insert(destination, value);
                if let Some(solved_dependencies) = dependencies.remove(destination) {
                    for (dep_dest, dep_op) in solved_dependencies {
                        if let Some(value) = dep_op.calculate(&known_monkeys) {
                            todo.push_back((dep_dest, Operation::Assign(value)))
                        }
                    }
                }
            }
            _ => panic!("Unexpected operation in assignment queue"),
        }
    }

    *known_monkeys
        .get("humn")
        .expect("Could not find humn monkey")
}

fn parse_line(line: &str) -> IResult<&str, (&str, Operation)> {
    separated_pair(
        take(4usize),
        tag(": "),
        alt((parse_assign, parse_operation)),
    )(line)
}

fn parse_assign(assignment: &str) -> IResult<&str, Operation> {
    let (res, number) = complete::i64(assignment)?;
    IResult::Ok((res, Operation::Assign(number)))
}

fn parse_operation(operation: &str) -> IResult<&str, Operation> {
    let (res, (lhs, op, rhs)) = tuple((
        take_while(|x| is_alphabetic(x as u8)),
        parse_operator,
        take_while(|x| is_alphabetic(x as u8)),
    ))(operation)?;

    IResult::Ok((
        res,
        match op {
            '+' => Operation::Add(lhs, rhs),
            '-' => Operation::Subtract(lhs, rhs),
            '*' => Operation::Multiply(lhs, rhs),
            '/' => Operation::Divide(lhs, rhs),
            _ => unreachable!(),
        },
    ))
}

fn parse_operator(operator: &str) -> IResult<&str, char> {
    delimited(
        complete::char(' '),
        alt((
            complete::char('+'),
            complete::char('-'),
            complete::char('*'),
            complete::char('/'),
        )),
        complete::char(' '),
    )(operator)
}

fn read_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = read_data("data/21-example.txt");
        assert_eq!(part_one(&data), 152);
    }

    #[test]
    fn test_part_two() {
        let data = read_data("data/21-example.txt");
        assert_eq!(part_two(&data), 301);
    }
}
