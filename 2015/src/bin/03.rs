use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("data/03.txt").expect("Could not read datafile");

    let mut current: (i32, i32) = (0, 0);
    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    coords.insert(current);

    data.lines().for_each(|line| {
        line.chars().for_each(|character| {
            if character == '^' {
                current.1 += 1;
            } else if character == '>' {
                current.0 += 1;
            } else if character == 'v' {
                current.1 -= 1;
            } else {
                current.0 -= 1;
            }
            coords.insert(current);
        })
    });

    let num_houses = coords.len();

    println!("Part 1: {}", num_houses);

    let mut current: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    let mut index = 0;
    let mut coords: HashSet<(i32, i32)> = HashSet::new();

    data.lines().for_each(|line| {
        line.chars().for_each(|character| {
            let person = index % 2;
            if character == '^' {
                current[person].1 += 1;
            } else if character == '>' {
                current[person].0 += 1;
            } else if character == 'v' {
                current[person].1 -= 1;
            } else {
                current[person].0 -= 1;
            }
            coords.insert(current[person]);
            index += 1;
        })
    });

    let num_houses = coords.len();

    println!("Part 2: {}", num_houses);
}
