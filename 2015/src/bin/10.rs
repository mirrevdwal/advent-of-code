use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data/10.txt").expect("Could not read datafile");
    let mut input = data.lines().next().expect("Datafile was empty").to_owned();

    for _ in 1..=40 {
	input = look_say(input);
    }

    println!("Part 1: {}", input.len());

    for _ in 1..=10 {
	input = look_say(input);
    }

    println!("Part 2: {}", input.len());
}

fn look_say(input: String) -> String {
    let mut characters = input.chars();
    let mut last_char: char = characters.next().expect("Input was empty");

    let mut output: String = String::new();
    let mut counter: u32 = 1;

    characters.for_each(|character| {
	if character == last_char {
	    counter += 1;
	} else {
	    output.push_str(&counter.to_string());
	    output.push(last_char);
	    last_char = character;
	    counter = 1;
	}
    });

    output.push_str(&counter.to_string());
    output.push(last_char);

    output
}
