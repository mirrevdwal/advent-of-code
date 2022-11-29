fn main() {
    let data: String = std::fs::read_to_string("data/04.txt").expect("Could not read datafile");

    let string: &str = data.lines().next().expect("Could not find first line");

    part_one(string);
    part_two(string);
}

fn part_one(data: &str) {
    let mut number: u32 = 1;
    loop {
        let hash = md5::compute(format!("{data}{number}"));
        if format!("{:?}", hash).chars().take(5).all(|x| x == '0') {
            break;
        }
        number += 1;
    }

    println!("Part 1: {number}");
}

fn part_two(data: &str) {
    let mut number: u32 = 1;
    loop {
        let hash = md5::compute(format!("{data}{number}"));
        if format!("{:?}", hash).chars().take(6).all(|x| x == '0') {
            break;
        }
        number += 1;
    }

    println!("Part 2: {number}");
}
