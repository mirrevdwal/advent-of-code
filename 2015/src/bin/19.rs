use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

fn main() {
    let data = read_to_string("data/19.txt").expect("Could not read datafile");

    part_one(data.clone());
    part_two(data);
}

fn part_one(data: String) {
    let mut replacement_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut lines = data.lines();
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let parts = line.split_once(" => ")?;
            Some((parts.0.to_owned(), parts.1.to_owned()))
        })
        .for_each(|(start, end)| {
            replacement_map
                .entry(start)
                .or_insert_with(Vec::new)
                .push(end);
        });

    let replacements = replacement_map.into_iter().collect::<Vec<_>>();

    let start_molecule = lines
        .next()
        .expect("No start molecule found after empty line");

    let replaced_strings = get_replacements(start_molecule, &replacements);
    let answer = replaced_strings.iter().unique().count();

    println!("Part 1: {:?}", answer);
}

fn part_two(data: String) {
    let mut replacement_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut lines = data.lines();
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let parts = line.split_once(" => ")?;
            Some((parts.0.to_owned(), parts.1.to_owned()))
        })
        .for_each(|(start, end)| {
            replacement_map
                .entry(end)
                .or_insert_with(Vec::new)
                .push(start);
        });

    let replacements = replacement_map
        .into_iter()
        .sorted_by_key(|(left, _)| left.len())
        .rev()
        .collect::<Vec<_>>();

    let start_molecule = lines
        .next()
        .expect("No start molecule found after empty line")
        .to_owned();

    let mut queue: Vec<(usize, String)> = Vec::new();
    queue.push((0, start_molecule));

    let mut visited: HashSet<String> = HashSet::new();
    let medicine = String::from("e");

    let final_depth = 'outer: loop {
        let new_queue_items = if let Some((depth, molecule)) = queue.pop() {
            get_replacements(&molecule, &replacements)
                .into_iter()
                .map(move |new_molecule| (depth + 1, new_molecule))
        } else {
            break None;
        };

        for (depth, molecule) in new_queue_items {
            if molecule == medicine {
                break 'outer Some(depth);
            }

            if !visited.contains(&molecule) {
                visited.insert(molecule.clone());
                queue.push((depth, molecule));
            }
        }
    };

    println!(
        "Part 2: {:?}",
        final_depth.expect("Could not find final result")
    );
    println!("Created {} molecules", visited.len());
}

fn get_replacements(start_molecule: &str, replacements: &[(String, Vec<String>)]) -> Vec<String> {
    replacements
        .iter()
        .flat_map(|(start, end_vec)| {
            let indices = start_molecule.match_indices(start);
            indices.cartesian_product(end_vec)
        })
        .map(|((index, start), end)| {
            let mut new_molecule = start_molecule.to_owned();
            new_molecule.replace_range(index..index + start.chars().count(), end);
            new_molecule
        })
        .collect::<Vec<_>>()
}
