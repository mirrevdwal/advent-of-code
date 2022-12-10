use std::fs::read_to_string;

fn main() {
    let data = parse_data("data/08.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer}");

    let answer = part_two(&data);
    println!("Part 2: {answer}");
}

fn part_one(data: &str) -> usize {
    let heights = get_heights(data);
    let grid_height = heights.len();
    let grid_width = heights[0].len();

    heights
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, &height)| {
                    (0..*x).all(|x_val| heights[y][x_val] < height)
                        || (x + 1..grid_width).all(|x_val| heights[y][x_val] < height)
                        || (0..y).all(|y_val| heights[y_val][*x] < height)
                        || (y + 1..grid_height).all(|y_val| heights[y_val][*x] < height)
                })
                .count()
        })
        .sum()
}

fn part_two(data: &str) -> usize {
    let heights = get_heights(data);
    let grid_height = heights.len();
    let grid_width = heights[0].len();

    heights
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &height)| {
                    let left_iter = 0..x;
                    let left = left_iter.clone().count();
                    let mut left_shorter = left_iter
                        .rev()
                        .take_while(|&x_val| heights[y][x_val] < height)
                        .count();
                    if left_shorter != left {
                        left_shorter += 1;
                    };

                    let right_iter = x + 1..grid_width;
                    let right = right_iter.clone().count();
                    let mut right_shorter = right_iter
                        .take_while(|&x_val| heights[y][x_val] < height)
                        .count();
                    if right_shorter != right {
                        right_shorter += 1;
                    };

                    let up_iter = 0..y;
                    let up = up_iter.clone().count();
                    let mut up_shorter = up_iter
                        .rev()
                        .take_while(|&y_val| heights[y_val][x] < height)
                        .count();
                    if up_shorter != up {
                        up_shorter += 1;
                    };

                    let down_iter = y + 1..grid_height;
                    let down = down_iter.clone().count();
                    let mut down_shorter = down_iter
                        .take_while(|&y_val| heights[y_val][x] < height)
                        .count();
                    if down_shorter != down {
                        down_shorter += 1;
                    }
                    left_shorter * right_shorter * up_shorter * down_shorter
                })
                .max()
                .unwrap()
        })
        .max()
        .expect("Could not find tree with optimal scenic score")
}

fn get_heights(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = parse_data("data/08-example.txt");
        assert_eq!(part_one(&data), 21);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/08-example.txt");
        assert_eq!(part_two(&data), 8);
    }
}
