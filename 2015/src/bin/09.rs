use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let data = read_to_string("data/09.txt").expect("Could not read datafile");

    let mut locations: HashSet<&str> = HashSet::new();
    let mut paths: HashMap<(&str, &str), u32> = HashMap::new();

    data.lines().for_each(|line| {
        let mut words = line.split_whitespace();

        let location1 = words.next().expect("Could not find first location");
        let location2 = words.nth(1).expect("Could not find second location");
        let distance = words
            .nth(1)
            .expect("Could not find distance")
            .parse::<u32>()
            .expect("Could not parse distance as u32");

        locations.insert(location1);
        locations.insert(location2);
        paths.insert((location1, location2), distance);
        paths.insert((location2, location1), distance);
    });

    let locations = Vec::from_iter(locations);

    let shortest_path = locations
        .iter()
        .permutations(locations.len())
        .map(|combination| {
            combination
                .windows(2)
                .map(|pair| {
                    paths
                        .get(&(*pair[0], *pair[1]))
                        .cloned()
                        .unwrap_or(u32::MAX / locations.len() as u32)
                })
                .sum::<u32>()
        })
        .min()
        .expect("Could not find a minimum path");

    println!("Part 1: {}", shortest_path);

    let longest_path = locations
        .iter()
        .permutations(locations.len())
        .map(|combination| {
            combination
                .windows(2)
                .map(|pair| paths.get(&(*pair[0], *pair[1])).cloned().unwrap_or(0))
                .sum::<u32>()
        })
        .max()
        .expect("Could not find a minimum path");

    println!("Part 2: {}", longest_path);
}
