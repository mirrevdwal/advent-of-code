use std::{fs::read_to_string, iter::Peekable, str::Chars};

type Key = String;
type Pair = (Key, Token);

#[derive(Debug, PartialEq)]
enum Token {
    String(String),
    Number(isize),
    Array(Vec<Token>),
    Object(Vec<Pair>),
}

#[derive(Debug, PartialEq)]
enum ParserEnvironment {
    Global,
    String,
    Number,
    Array(ArrayEnvironment),
    Object(ObjectEnvironment),
}

#[derive(Debug, PartialEq)]
enum ArrayEnvironment {
    BeforeValue(Vec<Token>),
    AfterValue(Vec<Token>),
}

#[derive(Debug, PartialEq)]
enum ObjectEnvironment {
    BeforeKey(Vec<Pair>),
    Intermediate(Vec<Pair>),
    AfterValue(Vec<Pair>, Key),
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = read_to_string("data/12.txt").expect("Could not read datafile");

    let tokens = tokenize(&data).expect("No main token found");
    let mut token_stack: Vec<&Token> = vec![&tokens];

    let mut total: isize = 0;

    while let Some(token) = token_stack.pop() {
        match token {
            Token::Number(value) => total += value,
            Token::Array(values) => {
                for value in values {
                    token_stack.push(value);
                }
            }
            Token::Object(pairs) => {
                for (_key, value) in pairs {
                    token_stack.push(value);
                }
            }
            _ => {}
        }
    }

    println!("Part 1: {total}");
}

fn part_two() {
    let data = read_to_string("data/12.txt").expect("Could not read datafile");

    let tokens = tokenize(&data).expect("No main token found");
    let mut token_stack: Vec<&Token> = vec![&tokens];

    let mut total: isize = 0;

    while let Some(token) = token_stack.pop() {
        match token {
            Token::Number(value) => total += value,
            Token::Array(values) => {
                for value in values {
                    token_stack.push(value);
                }
            }
            Token::Object(pairs) => {
                if !pairs
                    .iter()
                    .any(|(_key, value)| value == &Token::String("red".to_string()))
                {
                    for (_key, value) in pairs {
                        token_stack.push(value);
                    }
                }
            }
            _ => {}
        }
    }

    println!("Part 2: {total}");
}

fn tokenize(input: &str) -> Option<Token> {
    let mut char_iter = input.chars().peekable();

    let mut parser_environment = ParserEnvironment::Global;
    let mut environment_stack: Vec<ParserEnvironment> = Vec::new();
    let mut previous_token: Option<Token> = None;

    loop {
        skip_whitespaces(&mut char_iter);
        match parser_environment {
            ParserEnvironment::Global => {
                // This environment should be entered once at the start of parsing and once at the end
                if let Some(&chr) = char_iter.peek() {
                    environment_stack.push(parser_environment);
                    if let Some(environment) = get_parse_environment(chr) {
                        // All environments apart from number have a character indicating that their
                        // environment is starting, such as '"', '{' and '['
                        if environment != ParserEnvironment::Number {
                            let _ = char_iter.next();
                        }
                        parser_environment = environment;
                    } else {
                        panic!("Parser environment could not be determined");
                    }
                    continue;
                } else {
                    break previous_token;
                }
            }

            ParserEnvironment::String => {
                let mut value = String::new();
                loop {
                    let opt_char = char_iter.next();
                    if let Some(chr) = opt_char {
                        match chr {
                            '"' => {
                                parser_environment = environment_stack
                                    .pop()
                                    .expect("Could not find parent environment");
                                previous_token = Some(Token::String(value));
                                break;
                            }
                            _ => value.push(chr),
                        }
                    } else {
                        panic!("Ran out of data in string environment");
                    }
                }
            }

            ParserEnvironment::Number => {
                let mut value = String::new();
                value.push(
                    char_iter
                        .next()
                        .expect("Unreachable, char was already peeked at"),
                );

                loop {
                    let opt_char = char_iter.peek();
                    if let Some(chr) = opt_char {
                        if chr.is_ascii_digit() {
                            value.push(*chr);
                            let _ = char_iter.next();
                        } else {
                            parser_environment = environment_stack
                                .pop()
                                .expect("Could not find parent environment");
                            previous_token = Some(Token::Number(
                                value.parse().expect("Could not parse number"),
                            ));
                            break;
                        }
                    } else {
                        parser_environment = environment_stack
                            .pop()
                            .expect("Could not find parent environment");
                        println!("Trying to parse as number: {:?}", value);
                        previous_token = Some(Token::Number(
                            value.parse().expect("Could not parse number"),
                        ));
                        break;
                    }
                }
            }

            ParserEnvironment::Array(ArrayEnvironment::BeforeValue(values)) => {
                let opt_char = char_iter.peek();
                if let Some(&chr) = opt_char {
                    if chr == ']' {
                        parser_environment = environment_stack
                            .pop()
                            .expect("Could not find parent environment");
                        let _ = char_iter.next();
                        previous_token = Some(Token::Array(values));
                    } else {
                        if let Some(new_environment) = get_parse_environment(chr) {
                            if new_environment != ParserEnvironment::Number {
                                let _ = char_iter.next();
                            }
                            environment_stack.push(ParserEnvironment::Array(
                                ArrayEnvironment::AfterValue(values),
                            ));
                            match new_environment {
                                ParserEnvironment::String => {
                                    parser_environment = ParserEnvironment::String;
                                }
                                ParserEnvironment::Number => {
                                    parser_environment = ParserEnvironment::Number;
                                }
                                ParserEnvironment::Array(new_environment) => {
                                    parser_environment = ParserEnvironment::Array(new_environment);
                                }
                                ParserEnvironment::Object(new_environment) => {
                                    parser_environment = ParserEnvironment::Object(new_environment);
                                }
                                ParserEnvironment::Global => unreachable!(),
                            }
                        } else {
                            panic!("No valid parser environment found in array");
                        }
                        continue;
                    }
                } else {
                    panic!("Ran out of data in array environment")
                }
            }

            ParserEnvironment::Array(ArrayEnvironment::AfterValue(mut values)) => {
                if let Some(previous) = previous_token {
                    values.push(previous);
                } else {
                    panic!("No previous values found in last token on value stack");
                }
                previous_token = None;

                match char_iter
                    .next()
                    .expect("Ran out of data in open array environment")
                {
                    ',' => {
                        parser_environment =
                            ParserEnvironment::Array(ArrayEnvironment::BeforeValue(values));
                    }
                    ']' => {
                        parser_environment = environment_stack
                            .pop()
                            .expect("Could not find parent environment");
                        previous_token = Some(Token::Array(values));
                    }
                    _ => panic!("Encountered unexpected charater after value in array"),
                }
            }

            ParserEnvironment::Object(ObjectEnvironment::BeforeKey(pairs)) => {
                if let Some(chr) = char_iter.next() {
                    if chr == '}' {
                        parser_environment = environment_stack
                            .pop()
                            .expect("Could not find parent environment");
                        previous_token = Some(Token::Object(pairs));
                    } else if let Some(new_environment) = get_parse_environment(chr) {
                        if new_environment == ParserEnvironment::String {
                            environment_stack.push(ParserEnvironment::Object(
                                ObjectEnvironment::Intermediate(pairs),
                            ));
                            parser_environment = ParserEnvironment::String;
                        } else {
                            panic!("Expect object key to be a string");
                        }
                    } else {
                        panic!("No valid parser environment found in object");
                    }
                } else {
                    panic!("No characters left inside object environment");
                }
            }

            ParserEnvironment::Object(ObjectEnvironment::Intermediate(pairs)) => {
                let key = if let Some(Token::String(key)) = previous_token {
                    if let Some(chr) = char_iter.next() {
                        if chr != ':' {
                            panic!("Illegal character after key in object");
                        }
                        skip_whitespaces(&mut char_iter);
                        key
                    } else {
                        panic!("No character found after key in object");
                    }
                } else {
                    panic!("No key found before trying to parse value in object");
                };

                previous_token = None;

                if let Some(&chr) = char_iter.peek() {
                    if let Some(new_environment) = get_parse_environment(chr) {
                        if new_environment != ParserEnvironment::Number {
                            let _ = char_iter.next();
                        }

                        environment_stack.push(ParserEnvironment::Object(
                            ObjectEnvironment::AfterValue(pairs, key),
                        ));

                        match new_environment {
                            ParserEnvironment::String => {
                                parser_environment = ParserEnvironment::String;
                            }
                            ParserEnvironment::Number => {
                                parser_environment = ParserEnvironment::Number;
                            }
                            ParserEnvironment::Array(new_environment) => {
                                parser_environment = ParserEnvironment::Array(new_environment);
                            }
                            ParserEnvironment::Object(new_environment) => {
                                parser_environment = ParserEnvironment::Object(new_environment);
                            }
                            ParserEnvironment::Global => unreachable!(),
                        }
                    } else {
                        panic!("No valid parser environment found in array");
                    }
                } else {
                    panic!("No characters left inside object environment");
                }
                continue;
            }

            ParserEnvironment::Object(ObjectEnvironment::AfterValue(mut pairs, key)) => {
                if let Some(previous) = previous_token {
                    pairs.push((key, previous));
                } else {
                    panic!("No previous values found in last token on value stack");
                }
                previous_token = None;

                match char_iter
                    .next()
                    .expect("Ran out of data in open object environment")
                {
                    ',' => {
                        parser_environment =
                            ParserEnvironment::Object(ObjectEnvironment::BeforeKey(pairs));
                    }
                    '}' => {
                        parser_environment = environment_stack
                            .pop()
                            .expect("Could not find parent environment");
                        previous_token = Some(Token::Object(pairs));
                    }
                    _ => panic!("Encountered unexpected charater after value in object"),
                }
            }
        };
    }
}

fn get_parse_environment(character: char) -> Option<ParserEnvironment> {
    match character {
        '"' => Some(ParserEnvironment::String),
        _ if character.is_ascii_digit() => Some(ParserEnvironment::Number),
        '-' => Some(ParserEnvironment::Number),
        '[' => Some(ParserEnvironment::Array(ArrayEnvironment::BeforeValue(
            Vec::new(),
        ))),
        '{' => Some(ParserEnvironment::Object(ObjectEnvironment::BeforeKey(
            Vec::new(),
        ))),
        _ => None,
    }
}

fn skip_whitespaces(iter: &mut Peekable<Chars>) {
    while let Some(next) = iter.peek() {
        if next.is_whitespace() {
            let _ = iter.next();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{tokenize, Token};

    #[test]
    fn tokenize_string() {
        assert_eq!(tokenize("\"hi\""), Some(Token::String("hi".to_string())));
    }

    #[test]
    fn tokenize_number() {
        assert_eq!(tokenize("34"), Some(Token::Number(34)));
    }

    #[test]
    fn tokenize_negative_number() {
        assert_eq!(tokenize("-234"), Some(Token::Number(-234)));
    }

    #[test]
    fn tokenize_emtpy_array() {
        assert_eq!(tokenize("[]"), Some(Token::Array(Vec::new())));
    }

    #[test]
    fn tokenize_single_number_array() {
        assert_eq!(tokenize("[5]"), Some(Token::Array(vec![Token::Number(5)])));
    }

    #[test]
    fn tokenize_multiple_number_array() {
        assert_eq!(
            tokenize("[13,-8]"),
            Some(Token::Array(vec![Token::Number(13), Token::Number(-8)]))
        );
    }

    #[test]
    fn tokenize_single_string_array() {
        assert_eq!(
            tokenize("[\"lonely\"]"),
            Some(Token::Array(vec![Token::String("lonely".to_string())]))
        );
    }

    #[test]
    fn tokenize_multiple_string_array() {
        assert_eq!(
            tokenize("[\"better\", \"together\"]"),
            Some(Token::Array(vec![
                Token::String("better".to_string()),
                Token::String("together".to_string())
            ]))
        );
    }

    #[test]
    fn tokenize_nested_array() {
        assert_eq!(
            tokenize("[[]]"),
            Some(Token::Array(vec![Token::Array(Vec::new())]))
        );

        assert_eq!(
            tokenize("[5, [\"nested\", -10]]"),
            Some(Token::Array(vec![
                Token::Number(5),
                Token::Array(vec![
                    Token::String("nested".to_string()),
                    Token::Number(-10)
                ])
            ]))
        );
    }

    #[test]
    fn tokenize_with_whitespace() {
        assert_eq!(
            tokenize(" [3,  5]"),
            Some(Token::Array(vec![Token::Number(3), Token::Number(5)]))
        );

        assert_eq!(
            tokenize("\"string with whitespaces\""),
            Some(Token::String("string with whitespaces".to_string()))
        );
    }

    #[test]
    fn tokenize_empty_object() {
        assert_eq!(tokenize("{}"), Some(Token::Object(Vec::new())));
    }

    #[test]
    fn tokenize_object() {
        assert_eq!(
            tokenize("{\"a\": 12}"),
            Some(Token::Object(vec![("a".to_string(), Token::Number(12))]))
        );

        assert_eq!(
            tokenize("{\"a\": -9, \"boo\": \"false\"}"),
            Some(Token::Object(vec![
                ("a".to_string(), Token::Number(-9)),
                ("boo".to_string(), Token::String("false".to_string()))
            ]))
        );
    }

    #[test]
    fn tokenize_nested_object() {
        assert_eq!(
            tokenize("{\"foo\": -3, \"bar\": {\"cat\": \"red\", \"dog\": 8}}"),
            Some(Token::Object(vec![
                ("foo".to_string(), Token::Number(-3)),
                (
                    "bar".to_string(),
                    Token::Object(vec![
                        ("cat".to_string(), Token::String("red".to_string())),
                        ("dog".to_string(), Token::Number(8))
                    ])
                )
            ]))
        );
    }

    #[test]
    fn tokenize_object_in_array() {
        assert_eq!(
            tokenize("[\"a\", -8, {\"foo\": 6}, \"b\"]"),
            Some(Token::Array(vec![
                Token::String("a".to_string()),
                Token::Number(-8),
                Token::Object(vec![("foo".to_string(), Token::Number(6))]),
                Token::String("b".to_string())
            ]))
        );
    }

    #[test]
    fn tokenize_array_in_object() {
        assert_eq!(
            tokenize("{\"a\": [\"boo\", -8], \"foo\": 2}"),
            Some(Token::Object(vec![
                (
                    "a".to_string(),
                    Token::Array(vec![Token::String("boo".to_string()), Token::Number(-8)])
                ),
                ("foo".to_string(), Token::Number(2))
            ]))
        )
    }
}
