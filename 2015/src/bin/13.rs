use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let data = read_to_string("data/13.txt").expect("Could not read datafile");

    let regex =
        Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();
    let entries = parse_data(data, regex);

    let scores = entries
        .iter()
        .map(|entry| {
            let person1 = &entry[0];
            let win = &entry[1];
            let number = entry[2]
                .parse::<usize>()
                .expect("Could not parse start column as integer");
            let person2 = &entry[3];

            let score: i32 = match win.as_str() {
                "gain" => number as i32,
                "lose" => -(number as i32),
                _ => panic!("Unexpected word indicating win/loss"),
            };

            ((person1.to_string(), person2.to_string()), score)
        })
        .collect::<HashMap<(String, String), i32>>();

    let names = scores
        .keys()
        .map(|(name1, _name2)| name1)
        .unique()
        .collect::<Vec<_>>();

    let total = part_one(names.clone(), scores.clone());
    println!("Part 1: {:?}", total);

    let total = part_two(names, scores.clone());
    println!("Part 2: {:?}", total);
}

fn part_one(names: Vec<&String>, scores: HashMap<(String, String), i32>) -> i32 {
    names
        .iter()
        .permutations(names.len())
        .map(|mut circle| {
            circle.push(circle[0]);
            circle
                .windows(2)
                .map(|names| {
                    let &name1 = &names[0];
                    let &name2 = &names[1];
                    let score1 = scores
                        .get(&(name1.to_string(), name2.to_string()))
                        .expect(&format!("Could not find score for {name1}, {name2}"));
                    let score2 = scores
                        .get(&(name2.to_string(), name1.to_string()))
                        .expect(&format!("Could not find score for {name2}, {name1}"));

                    score1 + score2
                })
                .sum::<i32>()
        })
        .max()
        .expect("Could not find maximum score")
}

fn part_two(names: Vec<&String>, scores: HashMap<(String, String), i32>) -> i32 {
    names
        .iter()
        .permutations(names.len())
        .map(|mut circle| {
	    let mut worst_pair = i32::MAX;
            circle.push(circle[0]);
            circle
                .windows(2)
                .map(|names| {
                    let &name1 = &names[0];
                    let &name2 = &names[1];
                    let score1 = scores
                        .get(&(name1.to_string(), name2.to_string()))
                        .expect(&format!("Could not find score for {name1}, {name2}"));
                    let score2 = scores
                        .get(&(name2.to_string(), name1.to_string()))
                        .expect(&format!("Could not find score for {name2}, {name1}"));

		    if score1 + score2 < worst_pair {
			worst_pair = score1 + score2;
		    }

                    score1 + score2
                })
                .sum::<i32>() - worst_pair
        })
        .max()
        .expect("Could not find maximum score")
}

fn parse_data(data: String, regex: Regex) -> Vec<Vec<String>> {
    data.lines()
        .map(|line| {
            let capture = regex
                .captures_iter(line)
                .nth(0)
                .expect("Entry did not match expected format");
            let person1: String = capture[1].to_owned();
            let win: String = capture[2].to_owned();
            let number: String = capture[3].to_owned();
            let person2: String = capture[4].to_owned();

            vec![person1, win, number, person2]
        })
        .collect()
}
