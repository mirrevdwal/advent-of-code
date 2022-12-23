use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};

type Coord = (u32, u32);

enum Material {
    Rock,
    Sand,
}

fn main() {
    let data = parse_data("data/14.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let start_coord = (500, 0);
    let (mut occupied, lowest_rock) = get_occupied(data);

    let mut sand_count = 0;
    'outer: loop {
        let (mut sand_x, mut sand_y) = start_coord;

        loop {
            if sand_y >= lowest_rock {
                break 'outer;
            }
            if !occupied.contains_key(&(sand_x, sand_y + 1)) {
                sand_y += 1;
            } else if !occupied.contains_key(&(sand_x - 1, sand_y + 1)) {
                sand_x -= 1;
                sand_y += 1;
            } else if !occupied.contains_key(&(sand_x + 1, sand_y + 1)) {
                sand_x += 1;
                sand_y += 1;
            } else {
                occupied.insert((sand_x, sand_y), Material::Sand);
                sand_count += 1;
                break;
            }
        }
    }

    sand_count
}

fn part_two(data: &str) -> usize {
    let start_coord = (500, 0);
    let (mut occupied, lowest_rock) = get_occupied(data);

    let mut sand_count = 0;
    'outer: loop {
        let (mut sand_x, mut sand_y) = start_coord;

        loop {
            // If the sand is in the level above the floor,
            // it can't move any further
            if sand_y > lowest_rock {
                occupied.insert((sand_x, sand_y), Material::Sand);
                sand_count += 1;
                break;
            }

            if !occupied.contains_key(&(sand_x, sand_y + 1)) {
                sand_y += 1;
            } else if !occupied.contains_key(&(sand_x - 1, sand_y + 1)) {
                sand_x -= 1;
                sand_y += 1;
            } else if !occupied.contains_key(&(sand_x + 1, sand_y + 1)) {
                sand_x += 1;
                sand_y += 1;
            } else {
                occupied.insert((sand_x, sand_y), Material::Sand);
                sand_count += 1;
                if (sand_x, sand_y) == start_coord {
                    break 'outer;
                }
                break;
            }
        }
    }

    sand_count
}

fn get_occupied(input: &str) -> (HashMap<Coord, Material>, u32) {
    let mut occupied: HashMap<Coord, Material> = HashMap::new();

    let paths = input
        .lines()
        .map(|line| {
            let (_res, path) = parse_path(line).expect("Could not parse path from line");
            path
        })
        .collect::<Vec<_>>();

    let mut lowest_rock = 0;
    for rock_coord in paths.iter().flat_map(|path| coords_from_path(path)) {
        occupied.insert(rock_coord, Material::Rock);
        lowest_rock = u32::max(lowest_rock, rock_coord.1);
    }

    (occupied, lowest_rock)
}

fn coords_from_path(path: &[Coord]) -> impl Iterator<Item = Coord> + '_ {
    path.windows(2).flat_map(|coords| {
        let from = coords[0];
        let to = coords[1];

        let min_x = u32::min(from.0, to.0);
        let max_x = u32::max(from.0, to.0);
        let min_y = u32::min(from.1, to.1);
        let max_y = u32::max(from.1, to.1);

        (min_x..=max_x).cartesian_product(min_y..=max_y)
    })
}

fn parse_path(line: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, complete::char(','), complete::u32),
    )(line)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_parse_path() {
        let path = "498,4 -> 498,6 -> 496,6";
        let (_res, parsed_path) = parse_path(path).expect("Could not parse path from line");
        assert_eq!(parsed_path, vec![(498, 4), (498, 6), (496, 6)]);
    }

    #[test]
    fn test_coords_from_path() {
        let (_res, path) =
            parse_path("498,4 -> 498,6 -> 496,6").expect("Could not parse path from line");
        let coords = coords_from_path(&path).collect::<HashSet<Coord>>();
        assert_eq!(coords.len(), 5);
        assert!(coords.contains(&(498, 4)));
        assert!(coords.contains(&(498, 5)));
        assert!(coords.contains(&(498, 6)));
        assert!(coords.contains(&(497, 6)));
        assert!(coords.contains(&(497, 6)));
    }

    #[test]
    fn test_part_one() {
        let data = parse_data("data/14-example.txt");
        assert_eq!(part_one(&data), 24);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/14-example.txt");
        assert_eq!(part_two(&data), 93);
    }
}
