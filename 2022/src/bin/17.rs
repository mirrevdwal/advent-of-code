use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    fs::read_to_string,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct State {
    rock_index: usize,
    jet_index: usize,
    floor: Vec<isize>,
}

enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn get_stride(&self, width: isize) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            Direction::Down => -(width as isize),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            _ => Err(format!("Unexpected direction character found: '{}'", value).into()),
        }
    }
}

struct Chamber {
    active: Vec<isize>,
    filled: BTreeSet<isize>,
    width: isize,
    height: isize,
    floor: Vec<isize>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height + 8).rev() {
            for x in 0..self.width {
                if self.filled.contains(&(self.width * y + x)) {
                    write!(f, "#")?;
                } else if self.active.contains(&(self.width * y + x)) {
                    write!(f, "@")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Chamber {
    fn get_rocks() -> [Vec<isize>; 5] {
        [
            vec![0, 1, 2, 3],
            vec![1, 7, 8, 9, 15],
            vec![0, 1, 2, 9, 16],
            vec![0, 7, 14, 21],
            vec![0, 1, 7, 8],
        ]
    }

    fn new(width: isize) -> Self {
        Chamber {
            active: Vec::new(),
            filled: (0..width).collect(),
            width,
            height: 0,
            floor: vec![0; width as usize],
        }
    }

    fn place_rock(&mut self, rock: Vec<isize>) {
        let offset = (self.height + 4) * self.width + 2;
        self.active.extend(rock.iter().map(|index| index + offset));
    }

    fn is_legal_move(&self, direction: &Direction) -> bool {
        let is_legal_stride = match direction {
            Direction::Left => !self.active.iter().any(|index| index % self.width == 0),
            Direction::Right => !self
                .active
                .iter()
                .any(|index| index % self.width == self.width - 1),
            _ => true,
        };

        let stride = direction.get_stride(self.width);
        is_legal_stride
            && self
                .active
                .iter()
                .all(|index| !self.filled.contains(&(index + stride)))
    }

    fn make_move(&mut self, direction: &Direction) {
        let stride = direction.get_stride(self.width);
        for index in self.active.iter_mut() {
            *index += stride;
        }
    }

    fn solidify(&mut self) {
        for index in self.active.iter() {
            let column = (index % self.width) as usize;
            self.floor[column] = isize::max(self.floor[column], index / self.width);
        }
        self.filled.extend(self.active.drain(..));
        self.height = *self.floor.iter().max().unwrap();
    }

    fn get_relative_height(&self) -> impl Iterator<Item = isize> + '_ {
        let min_height = self.floor.iter().min().unwrap();
        self.floor.iter().map(move |height| height - min_height)
    }
}

fn main() {
    let data = parse_data("data/17.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> isize {
    let jets: Vec<Direction> = data.chars().filter_map(|chr| chr.try_into().ok()).collect();
    let mut jet_index = 0;

    let rocks = Chamber::get_rocks();
    let num_rocks = 2022;

    let mut chamber = Chamber::new(7);

    for rock_index in 0..num_rocks {
        chamber.place_rock(rocks[rock_index % rocks.len()].clone());

        loop {
            if chamber.is_legal_move(&jets[jet_index % jets.len()]) {
                chamber.make_move(&jets[jet_index % jets.len()]);
            }

            jet_index += 1;

            if chamber.is_legal_move(&Direction::Down) {
                chamber.make_move(&Direction::Down);
            } else {
                break;
            }
        }

        chamber.solidify();
    }

    chamber.height
}

fn part_two(data: &str) -> usize {
    let jets: Vec<Direction> = data.chars().filter_map(|chr| chr.try_into().ok()).collect();
    let mut jet_index = 0;

    let rocks = Chamber::get_rocks();
    let num_rocks = 1_000_000_000_000;

    let mut states = BTreeMap::new();
    let mut chamber = Chamber::new(7);

    let mut extra_height = 0;

    let mut rock_index = 0;
    while rock_index < num_rocks {
        chamber.place_rock(rocks[rock_index % rocks.len()].clone());

        loop {
            if chamber.is_legal_move(&jets[jet_index % jets.len()]) {
                chamber.make_move(&jets[jet_index % jets.len()]);
            }

            jet_index += 1;

            if chamber.is_legal_move(&Direction::Down) {
                chamber.make_move(&Direction::Down);
            } else {
                break;
            }
        }

        chamber.solidify();
        rock_index += 1;

        let state = State {
            rock_index: rock_index % rocks.len(),
            jet_index: jet_index % jets.len(),
            floor: chamber.get_relative_height().collect(),
        };

        if extra_height == 0 {
            if let Some((old_rock_index, old_height)) = states.get(&state) {
                let cycle_length = rock_index - old_rock_index;
                let cycle_height = chamber.height - old_height;
                let cycles_remaining = (num_rocks - rock_index) / cycle_length;
                extra_height = cycles_remaining * cycle_height as usize;
                rock_index += cycles_remaining * cycle_length;
                continue;
            } else {
                states.insert(state, (rock_index, chamber.height));
            }
        }
    }

    chamber.height as usize + extra_height
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/17-example.txt");
        assert_eq!(part_one(&data), 3068);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/17-example.txt");
        assert_eq!(part_two(&data), 1514285714288);
    }
}
