use std::{collections::HashMap, fs::read_to_string};

enum PropertyType {
    Smaller,
    Equal,
    Greater,
}

fn main() {
    let data = read_to_string("data/16.txt").expect("Could not read datafile");

    let aunts = data
        .lines()
        .map(|line| {
            let parts = line
                .split_once(": ")
                .expect("Could not split name from line");
            let name = parts.0.to_string();

            let properties = parts
                .1
                .split(", ")
                .filter_map(|property| {
                    let (item, amount) = property
                        .split_once(": ")
                        .expect(&format!("Could not split propery {property}"));
                    let amount = amount.parse::<usize>().ok()?;
                    Some((item, amount))
                })
                .collect::<Vec<_>>();

            (name, properties)
        })
        .collect::<Vec<(String, Vec<_>)>>();

    let mut known_properties: HashMap<&str, (PropertyType, usize)> = HashMap::new();
    known_properties.insert("children", (PropertyType::Equal, 3));
    known_properties.insert("cats", (PropertyType::Greater, 7));
    known_properties.insert("samoyeds", (PropertyType::Equal, 2));
    known_properties.insert("pomeranians", (PropertyType::Smaller, 3));
    known_properties.insert("akitas", (PropertyType::Equal, 0));
    known_properties.insert("vizslas", (PropertyType::Equal, 0));
    known_properties.insert("goldfish", (PropertyType::Smaller, 5));
    known_properties.insert("trees", (PropertyType::Greater, 3));
    known_properties.insert("cars", (PropertyType::Equal, 2));
    known_properties.insert("perfumes", (PropertyType::Equal, 1));

    let correct_aunts = aunts
        .iter()
        .filter_map(|(name, properties)| {
            if properties.iter().any(|(property, amount)| {
                *amount
                    != known_properties
                        .get(property)
                        .expect(&format!("Could not find property {property}"))
                        .1
            }) {
                return None;
            }
	    Some(name)
        })
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", correct_aunts);

    let correct_aunts = aunts
        .iter()
        .filter_map(|(name, properties)| {
            if !properties.iter().all(|(property, amount)| {
                let (property_type, known_amount) = known_properties
                    .get(property)
                    .expect(&format!("Could not find property {property}"));

                match property_type {
                    PropertyType::Smaller => amount < known_amount,
                    PropertyType::Equal => amount == known_amount,
                    PropertyType::Greater => amount > known_amount,
                }
            }) {
		return None;
            }
	    Some(name)
        })
        .collect::<Vec<_>>();

    println!("Part 2: {:?}", correct_aunts);
}
