use itertools::{chain, iproduct, Itertools};
use std::{fs::read_to_string, iter::once};

fn main() {
    let data = read_to_string("data/21.txt").expect("Could not read datafile");
    let boss = parse_data(&data);

    let health = 100;

    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    let rings = [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let weapon_combinations = weapons.iter().combinations(1);
    let armor_combinations = chain!(armor.iter().combinations(1), once(vec![&(0, 0, 0)]));
    let ring_combinations = chain!(
        rings.iter().combinations(2),
        rings.iter().combinations(1),
        once(vec![&(0, 0, 0)])
    );

    let mut outfits = iproduct!(weapon_combinations, armor_combinations, ring_combinations)
        .map(|(weapon_vec, armor_vec, ring_vec)| {
            chain!(weapon_vec.iter(), armor_vec.iter(), ring_vec.iter())
                .fold((0, 0, 0), |acc, cur| {
                    (acc.0 + cur.0, acc.1 + cur.1, acc.2 + cur.2)
                })
        })
        .collect::<Vec<_>>();

    outfits.sort_by_key(|(cost, _, _)| *cost);

    let winning_outfit = outfits.iter().find(|(_, damage, armor)| {
        let boss_damage = boss.1 - armor;
        let player_damage = damage - boss.2;
        let boss_turns = health / boss_damage + if health % boss_damage == 0 { 0 } else { 1 };
        let player_turns = boss.0 / player_damage + if boss.0 % player_damage == 0 { 0 } else { 1 };
        player_turns <= boss_turns
    });

    println!("Part 1: {:?}", winning_outfit);

    let losing_outfit = outfits.iter().rev().find(|(_, damage, armor)| {
        let boss_damage = if boss.1 > *armor {boss.1 - armor} else {1};
        let player_damage = if *damage > boss.2 {damage - boss.2} else {1};
        let boss_turns = health / boss_damage + if health % boss_damage == 0 { 0 } else { 1 };
        let player_turns = boss.0 / player_damage + if boss.0 % player_damage == 0 { 0 } else { 1 };
        player_turns > boss_turns
    });

    println!("Part 2: {:?}", losing_outfit);
}

fn parse_data(data: &str) -> (usize, usize, usize) {
    data.lines()
        .map(|line| {
            line.split_once(": ")
                .expect("Could not split line")
                .1
                .parse::<usize>()
                .expect("Could not parse data to usize")
        })
        .collect_tuple()
        .expect("Could not collect data into tuple")
}
