use std::{collections::BinaryHeap, fs::read_to_string};

struct Route {
    cost: usize,
    index: usize,
}

impl Ord for Route {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Route {}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

struct Hill {
    heights: Vec<u32>,
    start_index: usize,
    end_index: usize,
    width: usize,
    height: usize,
}

impl TryFrom<&str> for Hill {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let width = value.lines().next().expect("Empty dataset for hill").len();
        let height = value.lines().count();

        let mut start = None;
        let mut end = None;

        let heights = value
            .lines()
            .flat_map(|line| line.chars())
            .enumerate()
            .map(|(index, character)| match character {
                'S' => {
                    start = Some(index);
                    0
                }
                'E' => {
                    end = Some(index);
                    25
                }
                chr => chr as u32 - 'a' as u32,
            })
            .collect();

        let start_index = start.ok_or("Could not find start")?;
        let end_index = end.ok_or("Could not find end")?;

        Ok(Hill {
            heights,
            start_index,
            end_index,
            width,
            height,
        })
    }
}

impl Hill {
    fn get_neighbours(&self, index: usize) -> Vec<usize> {
        let mut neighbours = Vec::with_capacity(4);

        let row = index / self.width;
        let col = index % self.width;

        if row > 0 {
            neighbours.push(index - self.width);
        }
        if row < self.height - 1 {
            neighbours.push(index + self.width);
        }
        if col > 0 {
            neighbours.push(index - 1);
        }
        if col < self.width - 1 {
            neighbours.push(index + 1);
        }

        neighbours
    }

    fn get_valid_neighbours(&self, index: usize) -> impl Iterator<Item = usize> + '_ {
        let height = self.heights[index];
        self.get_neighbours(index)
            .into_iter()
            .filter(move |&neighbour| self.heights[neighbour] <= height + 1)
    }

    fn get_lowest_points(&self) -> impl Iterator<Item = usize> + '_ {
        self.heights
            .iter()
            .enumerate()
            .filter_map(|(index, height)| (*height == 0).then_some(index))
    }
}

fn main() {
    let data = parse_data("data/12.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(input: &str) -> usize {
    let hill = Hill::try_from(input).expect("Could not parse hill");

    let mut start_queue = BinaryHeap::new();
    start_queue.push(Route {
        cost: 0,
        index: hill.start_index,
    });

    find_shortest_path(hill, start_queue).expect("Could not find shortest path")
}

fn part_two(input: &str) -> usize {
    let hill = Hill::try_from(input).expect("Could not parse hill");

    let mut start_queue = BinaryHeap::new();
    for lowest_point in hill.get_lowest_points() {
        start_queue.push(Route {
            cost: 0,
            index: lowest_point,
        });
    }

    find_shortest_path(hill, start_queue).expect("Could not find shortest path")
}

fn find_shortest_path(hill: Hill, mut queue: BinaryHeap<Route>) -> Option<usize> {
    let mut shortest_path = vec![usize::MAX; hill.heights.len()];

    while let Some(route) = queue.pop() {
        for neighbour in hill.get_valid_neighbours(route.index) {
            let new_cost = route.cost + 1;
            if neighbour == hill.end_index {
                return Some(new_cost);
            }

            if new_cost < shortest_path[neighbour] {
                shortest_path[neighbour] = new_cost;
                queue.push(Route {
                    cost: new_cost,
                    index: neighbour,
                })
            }
        }
    }

    None
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/12-example.txt");
        assert_eq!(part_one(&data), 31);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/12-example.txt");
        assert_eq!(part_two(&data), 29);
    }
}
