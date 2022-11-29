use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let data = std::fs::read_to_string("data/05.txt").expect("Could not read datafile");

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let bad_combinations = ["ab", "cd", "pq", "xy"];

    let nice = data
        .lines()
        .filter(|line| {
            for bad_combination in bad_combinations {
                if line.contains(bad_combination) {
                    return false;
                }
            }

            if !line.chars().zip(line.chars().skip(1)).any(|(x, y)| x == y) {
                return false;
            }

            if line
                .chars()
                .filter(|character| vowels.contains(character))
                .count()
                < 3
            {
                return false;
            };

            true
        })
        .count();

    println!("Part 1: {}", nice);
}

fn part_two() {
    let data = std::fs::read_to_string("data/05.txt").expect("Could not read datafile");

    let nice = data
        .lines()
        .filter(|line| {
            let characters: Vec<char> = line.chars().collect();

            if !characters.windows(3).any(|window| window[0] == window[2]) {
                return false;
            }

            let pairs: HashSet<_> = characters.windows(2).collect();
            let triplets = characters
                .windows(4)
                .filter(|window| {
                    window[0] == window[1] && window[1] == window[2] && window[2] != window[3]
                })
                .count();

            if pairs.len() + triplets == line.len() - 1 {
                return false;
            }

            true
        })
        .count();

    println!("Part 2: {}", nice);
}
