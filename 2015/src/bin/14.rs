use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data/14.txt").expect("Could not read datafile");

    let reindeer = data.lines().filter_map(|line| {
        let mut words = line.split_whitespace();

        let speed = words.nth(3)?.parse::<usize>().ok()?;
        let fly_time = words.nth(2)?.parse::<usize>().ok()?;
        let rest_time = words.nth(6)?.parse::<usize>().ok()?;

        Some((speed, fly_time, rest_time))
    });

    let total_time = 2503;

    // Part 1
    let answer = reindeer
        .clone()
        .map(|(speed, fly_time, rest_time)| distance_after(total_time, speed, fly_time, rest_time))
        .max()
        .expect("Could not find winning distance");

    println!("Part 1: {answer}");

    // Part 2
    let data = reindeer.collect::<Vec<_>>();
    let mut score = vec![0; data.len()];

    (1..=total_time).for_each(|time| {
        let distances = distances_after(time, &data);
        let max_distance = distances
            .iter()
            .max()
            .unwrap_or_else(|| panic!("Could not find maximum distance at time {time}"));

        distances.iter().enumerate().for_each(|(index, distance)| {
            if distance == max_distance {
                score[index] += 1;
            }
        })
    });

    let answer = score
        .into_iter()
        .max()
        .expect("Could not find maximum score");

    println!("Part 2: {answer}");
}

fn distance_after(total_time: usize, speed: usize, fly_time: usize, rest_time: usize) -> usize {
    let loop_time = fly_time + rest_time;
    let completed_loops = total_time / loop_time;
    let remaining_time = (total_time % loop_time).min(fly_time);

    (completed_loops * fly_time + remaining_time) * speed
}

fn distances_after(total_time: usize, data: &[(usize, usize, usize)]) -> Vec<usize> {
    data.iter()
        .map(|&(speed, fly_time, rest_time)| distance_after(total_time, speed, fly_time, rest_time))
        .collect::<Vec<_>>()
}
