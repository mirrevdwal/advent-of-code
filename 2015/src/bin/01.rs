fn main() {
    let data = std::fs::read_to_string("./data/01.txt").expect("Data file could not be found");

    let floor = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| if character == '(' { 1i32 } else { -1i32 })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Part 1: {}", floor);

    let mut index: u32 = 0;
    let mut current_floor: i32 = 0;
    for line in data.lines() {
        for character in line.chars() {
            if character == '(' {
                current_floor += 1;
            } else {
                current_floor -= 1;
            }
            index += 1;
            if current_floor == -1 {
                break;
            }
        }
    }

    println!("Part 2: {:?}", index);
}
