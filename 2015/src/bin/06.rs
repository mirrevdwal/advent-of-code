use regex::Regex;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = std::fs::read_to_string("data/06.txt").expect("Could not read datafile");

    let rows = 1000;
    let columns = 1000;
    let mut lights: Vec<Vec<bool>> = vec![vec![false; columns]; rows];

    let regex = Regex::new(r"(\w+ \w+|\w+) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let instructions = parse_data(data, regex);

    for instruction in instructions {
	let command = &instruction[0];
	let start_row = instruction[1].parse::<usize>().expect("Could not parse start row as integer");
	let start_col = instruction[2].parse::<usize>().expect("Could not parse start column as integer");
	let stop_row = instruction[3].parse::<usize>().expect("Could not parse stop row as integer");
	let stop_col = instruction[4].parse::<usize>().expect("Could not parse stop column as integer");

	for row in start_row..=stop_row {
	    for col in start_col..=stop_col {
		if command == "turn on" {
		    lights[row][col] = true;
		} else if command == "turn off" {
		    lights[row][col] = false;
		} else {
		    lights[row][col] = !lights[row][col]
		}
	    }
	}
    }

    let lights_on = lights.iter().flatten().filter(|&&light| light == true).count();

    println!("Part 1: {lights_on}");
}

fn part_two() {
    let data = std::fs::read_to_string("data/06.txt").expect("Could not read datafile");

    let rows = 1000;
    let columns = 1000;
    let mut lights: Vec<Vec<usize>> = vec![vec![0; columns]; rows];

    let regex = Regex::new(r"(\w+ \w+|\w+) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let instructions = parse_data(data, regex);

    for instruction in instructions {
	let command = &instruction[0];
	let start_row = instruction[1].parse::<usize>().expect("Could not parse start row as integer");
	let start_col = instruction[2].parse::<usize>().expect("Could not parse start column as integer");
	let stop_row = instruction[3].parse::<usize>().expect("Could not parse stop row as integer");
	let stop_col = instruction[4].parse::<usize>().expect("Could not parse stop column as integer");

	for row in start_row..=stop_row {
	    for col in start_col..=stop_col {
		if command == "turn on" {
		    lights[row][col] += 1;
		} else if command == "turn off" {
		    lights[row][col] = usize::max(1, lights[row][col]) - 1;
		} else {
		    lights[row][col] += 2;
		}
	    }
	}
    }

    let brightness = lights.iter().flatten().sum::<usize>();

    println!("Part 2: {brightness}");
}

fn parse_data(data: String, regex: Regex) -> Vec<Vec<String>> {
    data.lines().map(|line| {
	let capture = regex.captures_iter(line).nth(0).expect("Command did not match expected format");
	let command: String = capture[1].to_owned();
	let start_row: String = capture[2].to_owned();
	let start_col: String = capture[3].to_owned();
	let stop_row: String = capture[4].to_owned();
	let stop_col: String = capture[5].to_owned();

	vec![command, start_row, start_col, stop_row, stop_col]
    }).collect()
}
