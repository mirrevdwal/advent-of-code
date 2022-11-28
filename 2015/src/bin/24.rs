use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let data = read_to_string("data/24.txt").expect("Could not read datafile");

    let packages = data
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let total_weight = packages.iter().sum::<usize>();

    for num in 1..packages.len() {
        let combinations = packages.iter().combinations(num);
        let correct_combinations = combinations.filter(|combination| {
            (combination).into_iter().map(|&&x| x).sum::<usize>() == total_weight / 3
        });

        if let Some(lowest_entanglement) = correct_combinations
            .map(|combination| combination.iter().map(|&&x| x).product::<usize>())
            .min()
        {
            println!("Part 1: {lowest_entanglement}");
            break;
        }
    }

    for num in 1..packages.len() {
        let combinations = packages.iter().combinations(num);
        let correct_combinations = combinations.filter(|combination| {
            (combination).into_iter().map(|&&x| x).sum::<usize>() == total_weight / 4
        });

        if let Some(lowest_entanglement) = correct_combinations
            .map(|combination| combination.iter().map(|&&x| x).product::<usize>())
            .min()
        {
            println!("Part 2: {lowest_entanglement}");
            break;
        }
    }
}
