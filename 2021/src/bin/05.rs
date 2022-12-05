use itertools::Itertools;
use std::fs::read_to_string;

const OCEAN_DIMENSION: usize = 1000;

type Coord = (usize, usize);

enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(PartialEq)]
enum DiagonalStatus {
    Enabled,
    Disabled,
}

fn main() {
    let data = parse_data("data/05.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let mut ocean = vec![0; OCEAN_DIMENSION * OCEAN_DIMENSION];

    data.lines()
        .filter_map(|line| {
            let (from, to) = parse_line(line);
            get_direction(from, to, DiagonalStatus::Disabled)
        })
        .for_each(|((from, to), direction)| {
            for index in get_indices_in_range(from, to, direction) {
                ocean[index] += 1;
            }
        });

    ocean.into_iter().filter(|&val| val >= 2).count()
}

fn part_two(data: &str) -> usize {
    let mut ocean = vec![0; OCEAN_DIMENSION * OCEAN_DIMENSION];

    data.lines()
        .filter_map(|line| {
            let (from, to) = parse_line(line);
            get_direction(from, to, DiagonalStatus::Enabled)
        })
        .for_each(|((from, to), direction)| {
            for index in get_indices_in_range(from, to, direction) {
                ocean[index] += 1;
            }
        });

    ocean.into_iter().filter(|&val| val >= 2).count()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

fn parse_line(line: &str) -> (Coord, Coord) {
    line.split(" -> ")
        .map(|point| {
            point
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_tuple::<(_, _)>()
        .unwrap()
}

fn get_indices_in_range(
    from: Coord,
    to: Coord,
    direction: Direction,
) -> impl Iterator<Item = usize> {
    let (stride, length) = match direction {
        Direction::Horizontal => {
            let length = usize::max(from.0, to.0) - usize::min(from.0, to.0) + 1;
            (1, length)
        }
        Direction::Vertical => {
            let length = usize::max(from.1, to.1) - usize::min(from.1, to.1) + 1;
            (OCEAN_DIMENSION, length)
        }
        Direction::Diagonal => {
            let x_flipped = from.0 > to.0; // default: left to right
            let y_flipped = from.1 < to.1; // default: top to bottom

            let length = usize::max(from.0, to.0) - usize::min(from.0, to.0) + 1;

            let stride = match x_flipped ^ y_flipped {
                true => OCEAN_DIMENSION + 1,
                false => OCEAN_DIMENSION - 1,
            };

            (stride, length)
        }
    };

    let start_index = usize::min(get_index(from), get_index(to));

    (0..length).map(move |num| start_index + num * stride)
}

fn get_index(coord: Coord) -> usize {
    OCEAN_DIMENSION * coord.1 + coord.0
}

fn get_direction(
    from: Coord,
    to: Coord,
    diagonal: DiagonalStatus,
) -> Option<((Coord, Coord), Direction)> {
    if from.0 == to.0 {
        Some(((from, to), Direction::Vertical))
    } else if from.1 == to.1 {
        Some(((from, to), Direction::Horizontal))
    } else {
        (diagonal == DiagonalStatus::Enabled).then_some(((from, to), Direction::Diagonal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/05-example.txt");
        assert_eq!(5, part_one(&data));
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/05-example.txt");
        assert_eq!(12, part_two(&data));
    }
}
