fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = std::fs::read_to_string("data/08.txt").expect("Could not read datafile");

    let difference: usize = data
        .lines()
        .map(|line| {
            let code_string = line;
            let mut memory_string = String::new();

            let mut chars = line.chars();
            let first_char = chars.next().expect("Line is emtpy");
            if first_char != '"' {
                panic!("First character was no double quote");
            }

            loop {
                let next_char = chars.next().expect("Line ended before quote was closed");
                match next_char {
                    '\\' => {
                        let escaped_char =
                            chars.next().expect("No escaped character after backslash");
                        match escaped_char {
                            '\\' => memory_string.push('\\'),
                            '"' => memory_string.push('"'),
                            'x' => {
                                let hexadecimal: String = chars.by_ref().take(2).collect();
                                let hex_u32 = u32::from_str_radix(&hexadecimal, 16)
                                    .expect("Could not parse hex value");
                                let hex_char = char::from_u32(hex_u32)
                                    .expect("Could not convert u32 to character");
                                memory_string.push(hex_char);
                            }
                            _ => panic!("Unexpected escaped character"),
                        }
                    }
                    '"' => break,
                    _ => memory_string.push(next_char),
                }
            }
            code_string.chars().count() - memory_string.chars().count()
        })
        .sum();

    println!("Part 1: {:?}", difference);
}

fn part_two() {
    let data = std::fs::read_to_string("data/08.txt").expect("Could not read datafile");

    let difference: usize = data
        .lines()
        .map(|line| {
            let code_string = line;
            let mut encoded_string = String::new();
            encoded_string.push('"');

            let chars = line.chars();

	    for character in chars {
                match character {
                    '\\' => encoded_string.push_str("\\\\"),
                    '"' => encoded_string.push_str("\\\""),
                    _ => encoded_string.push(character),
                }
            }

            encoded_string.push('"');

            encoded_string.chars().count() - code_string.chars().count()
        })
        .sum();

    println!("Part 2: {:?}", difference);
}
