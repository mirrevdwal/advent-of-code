use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete, multi::separated_list1};

const NUM_DIMENSIONS: usize = 3;

fn main() {
    let data = parse_data("data/18.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let points: Vec<_> = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_res, point)| point)
        .collect();

    get_total_sides(points)
}

fn part_two(data: &str) -> usize {
    let points: HashSet<_> = data
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|(_res, point)| point)
        .collect();

    let mut ranges = Vec::with_capacity(NUM_DIMENSIONS);

    for dimension in 0..NUM_DIMENSIONS {
        let min = points.iter().map(|point| point[dimension]).min().unwrap() - 1;
        let max = points.iter().map(|point| point[dimension]).max().unwrap() + 1;

        ranges.push((min, max));
    }

    let starting_point = ranges.iter().map(|range| range.0).collect::<Vec<_>>();
    let mut stack = vec![starting_point];

    let mut evaluated = HashSet::new();
    let mut sides = 0;

    while let Some(point) = stack.pop() {
        let neighbours = get_neighbours(&point, &ranges);

        for neighbour in neighbours {
            if points.contains(&neighbour) {
                sides += 1;
                continue;
            }
            if !evaluated.contains(&neighbour) {
                stack.push(neighbour.clone());
                evaluated.insert(neighbour);
            }
        }
    }

    sides
}

fn get_neighbours(point: &[i16], ranges: &[(i16, i16)]) -> Vec<Vec<i16>> {
    let mut neighbours = Vec::new();
    for dimension in 0..NUM_DIMENSIONS {
        if point[dimension] > ranges[dimension].0 {
            let mut neighbour = point.to_owned();
            neighbour[dimension] -= 1;
            neighbours.push(neighbour);
        }
        if point[dimension] < ranges[dimension].1 {
            let mut neighbour = point.to_owned();
            neighbour[dimension] += 1;
            neighbours.push(neighbour);
        }
    }

    neighbours
}

fn get_total_sides(mut points: Vec<Vec<i16>>) -> usize {
    let mut total_sides = points.len() * 6;

    let dimensions = (0..NUM_DIMENSIONS).collect::<Vec<_>>();
    for dimension in dimensions.iter() {
        let fixed_dimensions: Vec<_> = dimensions
            .iter()
            .cloned()
            .filter(|&dim| dim != *dimension)
            .collect();

        let mut order = fixed_dimensions.clone();
        order.push(*dimension);

        points.sort_by_key(|point| get_sort_key(point, order.clone()));

        total_sides -= 2 * points
            .iter()
            .tuple_windows()
            .filter(|(point1, point2)| {
                fixed_dimensions
                    .iter()
                    .all(|&fixed_dim| point1[fixed_dim] == point2[fixed_dim])
                    && point1[*dimension] + 1 == point2[*dimension]
            })
            .count();
    }

    total_sides
}

fn get_sort_key(point: &[i16], order: impl IntoIterator<Item = usize>) -> u32 {
    let mut total = 0;
    for key in order {
        total = (total << 8) + point[key] as u32;
    }
    total
}

fn parse_line(line: &str) -> nom::IResult<&str, Vec<i16>> {
    separated_list1(tag(","), complete::i16)(line)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/18-example.txt");
        assert_eq!(part_one(&data), 64);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/18-example.txt");
        assert_eq!(part_two(&data), 58);
    }
}
