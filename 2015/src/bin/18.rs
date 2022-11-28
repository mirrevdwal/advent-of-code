use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data/18.txt").expect("Could not read datafile");

    let height = data.lines().count();
    let stride = data
        .lines()
        .next()
        .expect("Datafile was emtpy")
        .chars()
        .count();
    let lights = data
        .chars()
        .filter_map(|character| match character {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .collect::<Vec<bool>>();

    part_one(lights.clone(), height, stride);
    part_two(lights.clone(), height, stride);
}

fn part_one(mut lights: Vec<bool>, height: usize, stride: usize) {
    for _time in 0..100 {
	lights = lights.iter().enumerate().map(|(index, state)| {
	    let lit_neighbors = get_neighbors(index, height, stride).filter(|&neighbor_index| {
		lights[neighbor_index]
	    }).count();

	    if *state {
		lit_neighbors == 2 || lit_neighbors == 3
	    } else {
		lit_neighbors == 3
	    }
	}).collect::<Vec<_>>();
    }

    let answer = lights.iter().filter(|&state| *state).count();
    println!("Part 1: {}", answer)
}

fn part_two(mut lights: Vec<bool>, height: usize, stride: usize) {
    lights[0] = true;
    lights[stride - 1] = true;
    lights[(height - 1) * stride] = true;
    lights[(height - 1) * stride + stride - 1] = true;
    
    for _time in 0..100 {
	lights = lights.iter().enumerate().map(|(index, state)| {
	    let lit_neighbors = get_neighbors(index, height, stride).filter(|&neighbor_index| {
		lights[neighbor_index]
	    }).count();

	    if *state {
		lit_neighbors == 2 || lit_neighbors == 3
	    } else {
		lit_neighbors == 3
	    }
	}).collect::<Vec<_>>();

	lights[0] = true;
	lights[stride - 1] = true;
	lights[(height - 1) * stride] = true;
	lights[(height - 1) * stride + stride - 1] = true;
    }

    let answer = lights.iter().filter(|&state| *state).count();
    println!("Part 2: {}", answer)
}

fn get_neighbors(index: usize, height: usize, stride: usize) -> impl Iterator<Item = usize> {
    let vertical_range =
        (usize::max(index / stride, 1) - 1)..=(usize::min(index / stride, height - 2) + 1);
    let horizontal_range =
        (usize::max(index % stride, 1) - 1)..=(usize::min(index % stride, stride - 2) + 1);

    vertical_range
        .flat_map(move |y| {
            horizontal_range
                .clone()
                .map(|x| stride * y as usize + x as usize)
                .collect::<Vec<_>>()
        })
        .filter(move |&i| i != index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors() {
        let mut neighbors = get_neighbors(27, 6, 6);
        assert_eq!(neighbors.next(), Some(20));
        assert_eq!(neighbors.next(), Some(21));
        assert_eq!(neighbors.next(), Some(22));
        assert_eq!(neighbors.next(), Some(26));
        assert_eq!(neighbors.next(), Some(28));
        assert_eq!(neighbors.next(), Some(32));
        assert_eq!(neighbors.next(), Some(33));
        assert_eq!(neighbors.next(), Some(34));
        assert_eq!(neighbors.next(), None);
    }

    #[test]
    fn test_get_neighbors_top_border() {
        let mut neighbors = get_neighbors(1, 6, 6);
        assert_eq!(neighbors.next(), Some(0));
        assert_eq!(neighbors.next(), Some(2));
        assert_eq!(neighbors.next(), Some(6));
        assert_eq!(neighbors.next(), Some(7));
        assert_eq!(neighbors.next(), Some(8));
        assert_eq!(neighbors.next(), None);
    }
}
