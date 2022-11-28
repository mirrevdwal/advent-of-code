use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let data = read_to_string("data/17.txt").expect("Could not read datafile");

    let containers = data
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let eggnog = 150;

    let valid_combination_count = (1..=containers.len())
        .flat_map(|num_containers| {
            containers
                .iter()
                .combinations(num_containers)
                .map(|combination| combination.into_iter().sum::<usize>())
        })
        .filter(|&volume| volume == eggnog)
        .count();

    println!("Part 1: {:?}", valid_combination_count);

    let mut valid_combinations = Vec::new();
    for num_containers in 1..=containers.len() {
	valid_combinations = containers.iter().combinations(num_containers).filter(|combination| {
	    combination.iter().map(|&x| x).sum::<usize>() == eggnog
	}).collect::<Vec<_>>();
	if valid_combinations.len() > 0 {
	    break;
	}
    };

    println!("Part 2: {:?}", valid_combinations.len());
}
