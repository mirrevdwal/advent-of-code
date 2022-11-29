use std::{cmp::Reverse, collections::BinaryHeap, fs::read_to_string};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum PlayerType {
    Player,
    Boss,
}

#[derive(Clone, Debug)]
struct State {
    player_health: usize,
    boss_health: usize,
    mana: usize,
    cost: usize,
    effects: Vec<Effect>,
    player_at_turn: PlayerType,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        self.damage == other.damage
            && self.armor == other.armor
            && self.mana == other.mana
            && self.cost == other.cost
    }
}

#[derive(Clone, Debug)]
struct Effect {
    timer: usize,
    damage: usize,
    armor: usize,
    mana: usize,
    cost: usize,
}

#[derive(Clone)]
struct Spell {
    damage: usize,
    health: usize,
    cost: usize,
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Hard,
}

fn main() {
    let data = read_to_string("data/22.txt").expect("Could not read datafile");
    let boss = parse_data(&data);

    let health = 50;
    let mana = 500;

    let initial_state = State {
        player_health: health,
        boss_health: boss.0,
        mana,
        cost: 0,
        effects: Vec::new(),
        player_at_turn: PlayerType::Player,
    };

    let spells = get_spells();
    let effects = get_effects();

    let winning_cost = get_winning_cost(
        initial_state.clone(),
        boss,
        spells.clone(),
        effects.clone(),
        Mode::Normal,
    );
    println!("Part 1: {:?}", winning_cost);

    let winning_cost = get_winning_cost(initial_state, boss, spells, effects, Mode::Hard);
    println!("Part 2: {:?}", winning_cost);
}

fn parse_data(data: &str) -> (usize, usize) {
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

fn get_spells() -> Vec<Spell> {
    let missile = Spell {
        damage: 4,
        health: 0,
        cost: 53,
    };

    let drain = Spell {
        damage: 2,
        health: 2,
        cost: 73,
    };

    vec![missile, drain]
}

fn get_effects() -> Vec<Effect> {
    let shield = Effect {
        timer: 6,
        damage: 0,
        armor: 7,
        mana: 0,
        cost: 113,
    };

    let poison = Effect {
        timer: 6,
        damage: 3,
        armor: 0,
        mana: 0,
        cost: 173,
    };

    let recharge = Effect {
        timer: 5,
        damage: 0,
        armor: 0,
        mana: 101,
        cost: 229,
    };

    vec![shield, poison, recharge]
}

fn get_winning_cost(
    initial_state: State,
    boss: (usize, usize),
    spells: Vec<Spell>,
    effects: Vec<Effect>,
    mode: Mode,
) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(initial_state));

    loop {
        let current_state = queue.pop().expect("Queue is empty").0;

        let mut boss_health = current_state.boss_health;
        let mut mana = current_state.mana;
        let mut armor = 0;

        let mut health = current_state.player_health;

        if mode == Mode::Hard && current_state.player_at_turn == PlayerType::Player {
            if health <= 1 {
                continue;
            } else {
                health -= 1;
            }
        }

        let mut new_effects = Vec::new();

        for effect in current_state.effects {
            let timer = effect.timer - 1;
            if boss_health <= effect.damage {
                return current_state.cost;
            }
            boss_health -= effect.damage;
            armor += effect.armor;
            mana += effect.mana;

            if timer > 0 {
                new_effects.push(Effect {
                    timer,
                    damage: effect.damage,
                    armor: effect.armor,
                    mana: effect.mana,
                    cost: effect.cost,
                })
            }
        }

        match current_state.player_at_turn {
            PlayerType::Player => {
                for spell in spells.iter() {
                    if boss_health < spell.damage {
                        return current_state.cost + spell.cost;
                    }
                    if spell.cost > mana {
                        continue;
                    }
                    queue.push(Reverse(State {
                        player_health: health + spell.health,
                        boss_health: boss_health - spell.damage,
                        mana: mana - spell.cost,
                        cost: current_state.cost + spell.cost,
                        effects: new_effects.clone(),
                        player_at_turn: PlayerType::Boss,
                    }));
                }
                for effect in effects.iter() {
                    if new_effects.contains(effect) {
                        continue;
                    }
                    if effect.cost > mana {
                        continue;
                    }
                    let mut tmp = new_effects.clone();
                    tmp.push(effect.clone());
                    queue.push(Reverse(State {
                        player_health: health,
                        boss_health,
                        mana: mana - effect.cost,
                        cost: current_state.cost + effect.cost,
                        effects: tmp,
                        player_at_turn: PlayerType::Boss,
                    }))
                }
            }
            PlayerType::Boss => {
                let boss_damage = usize::max(boss.1, armor + 1) - armor;
                if health <= boss_damage {
                    continue;
                }
                queue.push(Reverse(State {
                    player_health: health - boss_damage,
                    boss_health,
                    mana,
                    cost: current_state.cost,
                    effects: new_effects,
                    player_at_turn: PlayerType::Player,
                }))
            }
        }
    }
}
