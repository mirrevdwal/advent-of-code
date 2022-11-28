use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data/20.txt").expect("Could not read datafile");

    let input = data
        .lines()
        .next()
        .expect("Datafile was empty")
        .parse::<usize>()
        .expect("Could not parse data as number (u32)");

    let multiplier = 10;
    let answer = part_one(input, multiplier);
    println!("Part 1: {}", answer);

    let multiplier = 11;
    let answer = part_two(input, multiplier);
    println!("Part 2: {}", answer);
}

fn part_one(input: usize, multiplier: usize) -> usize {
    let mut presents = vec![0; input / multiplier];

    for elf in 1..=(input / multiplier) {
        let mut house_number = elf;
        while house_number < input / multiplier {
            presents[house_number] += elf * multiplier;
            house_number += elf;
        }
    }

    presents
        .into_iter()
        .enumerate()
        .find(|(_number, presents)| presents > &input)
        .expect("Could not find house with enough presents").0
}

fn part_two(input: usize, multiplier: usize) -> usize {
    let mut presents = vec![0; input / multiplier];

    for elf in 1..=(input / multiplier) {
        let mut house_number = elf;
        while house_number < input / multiplier && house_number < 51* elf {
            presents[house_number] += elf * multiplier;
            house_number += elf;
        }
    }

    presents
        .into_iter()
        .enumerate()
        .find(|(_number, presents)| presents > &input)
        .expect("Could not find house with enough presents").0
}

