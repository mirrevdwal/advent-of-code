use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::fs::read_to_string;

type Coord = (i64, i64);

struct Sensor {
    position: Coord,
    range: i64,
}

impl Sensor {
    fn in_range(&self, coord: Coord) -> bool {
        manhattan_distance(self.position, coord) <= self.range
    }
}

fn main() {
    let data = parse_data("data/15.txt");

    let answer = part_one(&data, 2_000_000);
    println!("Part 1: {answer}");

    let answer = part_two(&data, (0, 4_000_000), (0, 4_000_000));
    println!("Part 2: {answer}");
}

fn part_one(data: &str, row: i64) -> i64 {
    let sensors = data.lines().map(|line| {
        let (sensor, beacon) = parse_line(line).expect("Could not parse line").1;
        Sensor {
            position: sensor,
            range: manhattan_distance(sensor, beacon),
        }
    });

    let mut excluded_ranges = sensors
        .filter_map(|sensor| {
            let distance_to_row = i64::abs(row - sensor.position.1);
            let excluded_distance = sensor.range - distance_to_row;

            (excluded_distance >= 0).then_some((
                sensor.position.0 - excluded_distance,
                sensor.position.0 + excluded_distance,
            ))
        })
        .collect::<Vec<_>>();

    excluded_ranges.sort();

    get_non_overlapping_ranges(excluded_ranges)
        .iter()
        .map(|(start, end)| end - start)
        .sum::<i64>()
}

fn part_two(data: &str, x_range: (i64, i64), y_range: (i64, i64)) -> i64 {
    let sensors = data
        .lines()
        .map(|line| {
            let (sensor, beacon) = parse_line(line).expect("Could not parse line").1;
            Sensor {
                position: sensor,
                range: manhattan_distance(sensor, beacon),
            }
        })
        .collect::<Vec<_>>();

    let pois = get_all_points_of_interest(&sensors);

    let valid_poi = pois
        .iter()
        .filter(|&poi| is_valid_coord(*poi, x_range, y_range))
        .find(|&poi| !sensors.iter().any(|sensor| sensor.in_range(*poi)))
        .expect("Could not find valid POI");

    valid_poi.0 * 4_000_000 + valid_poi.1
}

fn get_points_of_interest(from: &Sensor, to: &Sensor) -> Option<Vec<Coord>> {
    let distance = manhattan_distance(from.position, to.position);
    let total_range = from.range + to.range;

    if distance >= total_range {
        return None;
    }

    let distance_vec = (
        to.position.0 - from.position.0,
        to.position.1 - from.position.1,
    );

    let offset = total_range - distance;

    // Creates two points of interest, since the intersection is
    // not on one of the points on the grid
    if offset % 2 != 0 {
        return None;
    }

    // Intersects at a certain point on the grid
    let offset = offset / 2;

    let abs_horizontal_distance = i64::abs(distance_vec.0);
    let abs_vertical_distance = i64::abs(distance_vec.1);

    let horizontal_direction = match distance_vec.0.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    let vertical_direction = match distance_vec.1.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };

    let path_len = from.range - offset;

    // First, traverse horizontally (only vertically if necessary)
    // To reach POI you need to traverse vertically (away from 'to' sensor) once
    let horizontal_steps = i64::min(abs_horizontal_distance, path_len);
    let vertical_steps = i64::max(path_len - horizontal_steps, 0);

    let closest_x = from.position.0 + horizontal_steps * horizontal_direction;
    let closest_y = from.position.1 + vertical_steps * vertical_direction;

    let poi1 = if vertical_steps > 0 {
        let offset_direction = if horizontal_direction == 0 {
            -1
        } else {
            horizontal_direction
        };
        (closest_x + (offset + 1) * offset_direction, closest_y)
    } else {
        let offset_direction = if vertical_direction == 0 {
            -1
        } else {
            vertical_direction
        };
        (closest_x, closest_y - (offset + 1) * offset_direction)
    };

    // Then, traverse vertically (only horizontally if necessary)
    // Again, traverse horizontally (away from 'to' sensor) once to reach POI
    let vertical_steps = i64::min(abs_vertical_distance, path_len);
    let horizontal_steps = i64::max(path_len - vertical_steps, 0);

    let closest_x = from.position.0 + horizontal_steps * horizontal_direction;
    let closest_y = from.position.1 + vertical_steps * vertical_direction;

    let poi2 = if horizontal_steps > 0 {
        let offset_direction = if vertical_direction == 0 {
            1
        } else {
            vertical_direction
        };
        (closest_x, closest_y + (offset + 1) * offset_direction)
    } else {
        let offset_direction = if horizontal_direction == 0 {
            1
        } else {
            horizontal_direction
        };
        (closest_x - (offset + 1) * offset_direction, closest_y)
    };

    Some(vec![poi1, poi2])
}

fn get_all_points_of_interest(sensors: &[Sensor]) -> Vec<Coord> {
    // TODO: For each sensor: check whether it locks a possible unseen beacon in the corner
    // of the field. In that case, we are interested in the point in the corner.

    // For each combination of sensors: check whether they have an overlapping edge.
    // In that case, we are interested in points next to this intersection.
    sensors
        .iter()
        .tuple_combinations()
        .filter_map(|(from, to)| get_points_of_interest(from, to))
        .flatten()
        .collect()
}

fn is_valid_coord(coord: Coord, x_range: (i64, i64), y_range: (i64, i64)) -> bool {
    x_range.0 <= coord.0 && coord.0 <= x_range.1 && y_range.0 <= coord.1 && coord.1 <= y_range.1
}

/// Precondition: Assumes a sorted iterator of ranges
fn get_non_overlapping_ranges(ranges: impl IntoIterator<Item = (i64, i64)>) -> Vec<(i64, i64)> {
    let mut non_overlapping_ranges = Vec::new();
    let mut ranges_iter = ranges.into_iter();

    let (mut current_start, mut current_end) = ranges_iter
        .next()
        .expect("Empty range iterator encountered");

    for (start, end) in ranges_iter {
        if start > current_end {
            non_overlapping_ranges.push((current_start, current_end));
            current_start = start;
        }
        current_end = i64::max(end, current_end);
    }

    non_overlapping_ranges.push((current_start, current_end));
    non_overlapping_ranges
}

fn manhattan_distance(from: Coord, to: Coord) -> i64 {
    i64::abs(from.0 - to.0) + i64::abs(from.1 - to.1)
}

fn parse_line(line: &str) -> IResult<&str, (Coord, Coord)> {
    separated_pair(
        preceded(take_until("x="), parse_coord),
        tag(": "),
        preceded(take_until("x="), parse_coord),
    )(line)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let parsed_coords = parse_line(line).unwrap().1;
        assert_eq!(parsed_coords, ((2, 18), (-2, 15)));
    }

    #[test]
    fn test_get_non_overlapping_ranges() {
        let ranges = vec![(0, 3), (1, 12), (1, 3), (14, 18)];
        assert_eq!(get_non_overlapping_ranges(ranges), vec![(0, 12), (14, 18)]);
    }

    #[test]
    fn test_get_poi() {
        let sensor1 = Sensor {
            position: (6, 8),
            range: 4,
        };
        let sensor2 = Sensor {
            position: (7, 4),
            range: 3,
        };

        let poi = get_points_of_interest(&sensor1, &sensor2).unwrap();
        assert_eq!(poi.len(), 2);
        assert!(poi.contains(&(4, 5)));
        assert!(poi.contains(&(9, 6)));
    }

    // This is one of the edgecases the function does not handle well.
    // It can find one of the POI, but would need optimalization to find all.
    #[test]
    fn test_get_poi_edgecase() {
        let sensor1 = Sensor {
            position: (14, 17),
            range: 5,
        };
        let sensor2 = Sensor {
            position: (12, 14),
            range: 4,
        };

        let poi = get_points_of_interest(&sensor1, &sensor2).unwrap();
        assert!(poi.contains(&(9, 16)));
    }

    #[test]
    fn test_part_one() {
        let data = parse_data("data/15-example.txt");
        assert_eq!(part_one(&data, 10), 26);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/15-example.txt");
        assert_eq!(part_two(&data, (0, 20), (0, 20)), 56000011);
    }
}
