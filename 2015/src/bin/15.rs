use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let data = read_to_string("data/15.txt").expect("Could not read datafile");

    let regex = Regex::new(
        r"(\w+): capacity ([-]?\d+), durability ([-]?\d+), flavor ([-]?\d+), texture ([-]?\d+), calories ([-]?\d+)",
    )
    .expect("Could not create regex string");
    let ingredients = parse_data(data, regex);

    let total_teaspoons = 100;
    let total_ingredients = ingredients.len();
    let total_properties = ingredients[0].len();

    let combinations = get_combinations_sum(total_teaspoons, ingredients.len());

    let best_score = combinations
        .iter()
        .map(|combination| {
            (0..(total_properties - 1))
                .map(|property_index| {
                    isize::max(
                        (0..total_ingredients)
                            .map(|ingredient_index| {
                                ingredients[ingredient_index][property_index]
                                    * combination[ingredient_index] as isize
                            })
                            .sum(),
                        0,
                    )
                })
                .product::<isize>()
        })
        .max()
        .expect("Could not find maximum score");

    println!("Part 1: {:?}", best_score);

    let best_score = combinations
        .into_iter()
        .map(|combination| {
            (0..(total_properties - 1))
                .filter_map(|property_index| {
                    let calories = (0..total_ingredients)
                        .map(|ingredient_index| {
                            ingredients[ingredient_index][total_properties - 1]
                                * combination[ingredient_index] as isize
                        })
                        .sum::<isize>();
                    (calories == 500).then(|| {
                        isize::max(
                            (0..total_ingredients)
                                .map(|ingredient_index| {
                                    ingredients[ingredient_index][property_index]
                                        * combination[ingredient_index] as isize
                                })
                                .sum(),
                            0,
                        )
                    })
                })
                .product::<isize>()
        })
        .max()
        .expect("Could not find maximum score");

    println!("Part 2: {:?}", best_score);
}

fn parse_data(data: String, regex: Regex) -> Vec<Vec<isize>> {
    data.lines()
        .filter_map(|line| {
            let captures = regex.captures(line)?;

            let capacity = captures[2].parse::<isize>().ok()?;
            let durability = captures[3].parse::<isize>().ok()?;
            let flavor = captures[4].parse::<isize>().ok()?;
            let texture = captures[5].parse::<isize>().ok()?;
            let calories = captures[6].parse::<isize>().ok()?;

            Some(vec![capacity, durability, flavor, texture, calories])
        })
        .collect::<Vec<Vec<_>>>()
}

fn get_combinations_sum(total: usize, depth: usize) -> Vec<Vec<usize>> {
    if depth == 1 {
        return vec![vec![total]];
    }

    (0..=total)
        .flat_map(|x| {
            let mut combinations = get_combinations_sum(total - x, depth - 1);
            combinations
                .iter_mut()
                .for_each(|combination| combination.push(x));
            combinations
        })
        .collect::<Vec<Vec<_>>>()
}
