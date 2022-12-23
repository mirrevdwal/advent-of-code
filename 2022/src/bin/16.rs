use std::fs::read_to_string;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until},
    character::{complete, is_digit},
    multi::separated_list1,
    sequence::{pair, preceded, tuple},
    IResult,
};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct State {
    position: usize,
    pressure: usize,
    flow_rate: usize,
    valves: Valves,
}

#[derive(Clone, Debug, Default)]
struct Valves {
    open_valves: usize,
}

impl Valves {
    fn get_closed_valves(&self, num_valves: usize) -> impl Iterator<Item = usize> + '_ {
        (0..num_valves).filter(|valve_index| !self.valve_is_open(*valve_index))
    }

    fn all_valves_open(&self, num_valves: usize) -> bool {
        self.open_valves == (1 << num_valves) - 1
    }

    fn valve_is_open(&self, valve_index: usize) -> bool {
        (self.open_valves >> valve_index) & 1 == 1
    }

    fn open_valve(&mut self, valve_index: usize) {
        self.open_valves |= 1 << valve_index;
    }
}

fn main() {
    let data = parse_data("data/16.txt");

    let answer = part_one(&data);
    println!("Part 1: {answer:?}");

    let answer = part_two(&data);
    println!("Part 2: {answer:?}");
}

fn part_one(data: &str) -> usize {
    let total_time = 30;

    // Parse valves from input data
    let all_valves = data
        .lines()
        .filter_map(|line| parse_input(line).ok())
        .map(|(_res, (valve, flow_rate, neighbours))| (valve, flow_rate as usize, neighbours))
        .collect::<Vec<_>>();

    // Get all-pair shortest paths
    let shortest_paths_all = get_shortest_paths(&all_valves);

    // Get original indices of flowing valves
    let flowing_valves = all_valves
        .iter()
        .enumerate()
        .filter_map(|(index, (_valve, flow_rate, _connected_valves))| {
            (*flow_rate != 0).then_some(index)
        })
        .collect::<Vec<_>>();

    // Get flow rate of flowing valves
    let flow_rates = flowing_valves
        .iter()
        .map(|index| all_valves[*index].1)
        .collect::<Vec<_>>();

    // Reduce shortest paths matrix to flowing valves only
    let mut distances = vec![vec![usize::MAX; flowing_valves.len()]; flowing_valves.len()];
    for (valve_index, original_index) in flowing_valves.iter().enumerate() {
        for (conn_valve_index, conn_original_index) in flowing_valves.iter().enumerate() {
            distances[valve_index][conn_valve_index] =
                shortest_paths_all[*original_index][*conn_original_index];
        }
    }

    // Get original index of starting valve (since it is not a flowing valve)
    let starting_index = all_valves
        .iter()
        .position(|(name, _flow_rate, _connected_valves)| *name == "AA")
        .expect("Could not find starting valve");

    // Get distance from starting valve to each flowing valve
    let starting_costs = flowing_valves
        .iter()
        .map(|index| shortest_paths_all[starting_index][*index]);

    // Create a queue with states to check for each time step
    let mut starting_queue = vec![vec![]; total_time];

    // Place initial valves into queue
    for (valve_index, path_len) in starting_costs.enumerate() {
        let mut state = State {
            position: valve_index,
            pressure: 0,
            flow_rate: flow_rates[valve_index],
            valves: Valves::default(),
        };
        state.valves.open_valve(valve_index);
        starting_queue[path_len + 1].push(state);
    }

    find_optimal_pressure(
        &mut starting_queue,
        &flow_rates,
        &flowing_valves,
        &distances,
        total_time,
    )
}

fn part_two(data: &str) -> usize {
    let total_time = 26;

    // Parse valves from input data
    let all_valves = data
        .lines()
        .filter_map(|line| parse_input(line).ok())
        .map(|(_res, (valve, flow_rate, neighbours))| (valve, flow_rate as usize, neighbours))
        .collect::<Vec<_>>();

    // Get all-pair shortest paths
    let shortest_paths_all = get_shortest_paths(&all_valves);

    // Get original indices of flowing valves
    let flowing_valves = all_valves
        .iter()
        .enumerate()
        .filter_map(|(index, (_valve, flow_rate, _connected_valves))| {
            (*flow_rate != 0).then_some(index)
        })
        .collect::<Vec<_>>();

    // Get original index of starting valve (since it is not a flowing valve)
    let starting_index = all_valves
        .iter()
        .position(|(name, _flow_rate, _connected_valves)| *name == "AA")
        .expect("Could not find starting valve");

    (1..(flowing_valves.len() / 2))
        .flat_map(|human_valves_num| {
            let human_valve_combinations = flowing_valves
                .iter()
                .cloned()
                .combinations(human_valves_num);
            human_valve_combinations.map(|human_valves| {
                let elephant_valves = flowing_valves
                    .iter()
                    .filter(|valve| !human_valves.contains(valve))
                    .cloned()
                    .collect::<Vec<_>>();

                // For each of the valve sets, repeat part 1
                [human_valves, elephant_valves]
                    .iter()
                    .map(|chosen_valves| {
                        // Get flow rate of chosen valves
                        let flow_rates = chosen_valves
                            .iter()
                            .map(|index| all_valves[*index].1)
                            .collect::<Vec<_>>();

                        // Reduce shortest paths matrix to flowing valves only
                        let mut distances =
                            vec![vec![usize::MAX; chosen_valves.len()]; chosen_valves.len()];
                        for (valve_index, original_index) in chosen_valves.iter().enumerate() {
                            for (conn_valve_index, conn_original_index) in
                                chosen_valves.iter().enumerate()
                            {
                                distances[valve_index][conn_valve_index] =
                                    shortest_paths_all[*original_index][*conn_original_index];
                            }
                        }

                        // Get distance from starting valve to each flowing valve
                        let starting_costs = chosen_valves
                            .iter()
                            .map(|index| shortest_paths_all[starting_index][*index]);

                        // Create a queue with states to check for each time step
                        let mut starting_queue = vec![vec![]; total_time];

                        // Place initial valves into queue
                        for (valve_index, path_len) in starting_costs.enumerate() {
                            let mut state = State {
                                position: valve_index,
                                pressure: 0,
                                flow_rate: flow_rates[valve_index],
                                valves: Valves::default(),
                            };
                            state.valves.open_valve(valve_index);
                            starting_queue[path_len + 1].push(state);
                        }

                        find_optimal_pressure(
                            &mut starting_queue,
                            &flow_rates,
                            chosen_valves,
                            &distances,
                            total_time,
                        )
                    })
                    .sum::<usize>()
            })
        })
        .max()
        .expect("Could not find optimal pressure")
}

fn find_optimal_pressure(
    queue: &mut [Vec<State>],
    flow_rates: &[usize],
    chosen_valves: &[usize],
    shortest_paths: &[Vec<usize>],
    total_time: usize,
) -> usize {
    let mut memo = vec![vec![vec![0; 1 << chosen_valves.len()]; chosen_valves.len()]; total_time];
    let mut optimal_pressure = 0;

    for time in 0..total_time {
        let time_left = total_time - time;
        let mut current_states = vec![];
        std::mem::swap(&mut current_states, &mut queue[time]);

        for state in current_states.drain(..) {
            // If all valves are open, all that's left to do is wait while the pressure builds
            if state.valves.all_valves_open(chosen_valves.len()) {
                optimal_pressure = usize::max(
                    optimal_pressure,
                    state.pressure + time_left * state.flow_rate,
                );
                continue;
            }

            // If the valve is open, there is nothing to do here; move to a different valve
            if state.valves.valve_is_open(state.position) {
                for neighbour in state.valves.get_closed_valves(chosen_valves.len()) {
                    let path_len = shortest_paths[state.position][neighbour];

                    if path_len >= time_left {
                        optimal_pressure = usize::max(
                            optimal_pressure,
                            state.pressure + time_left * state.flow_rate,
                        );
                        continue;
                    }
                    let new_time = time + path_len;
                    let mut new_state = state.clone();
                    new_state.position = neighbour;
                    new_state.pressure += path_len * state.flow_rate;

                    if memo[new_time][new_state.position][new_state.valves.open_valves]
                        < new_state.pressure
                    {
                        memo[new_time][new_state.position][new_state.valves.open_valves] =
                            new_state.pressure;
                        queue[new_time].push(new_state);
                    }
                }
            // If the valve is closed, open it
            // (There is no point in moving, since there would always be a shorter path to this valve)
            } else {
                if 1 >= time_left {
                    optimal_pressure = usize::max(
                        optimal_pressure,
                        state.pressure + time_left * state.flow_rate,
                    );
                    continue;
                }

                let new_time = time + 1;
                let mut new_state = state.clone();
                new_state.valves.open_valve(state.position);
                new_state.flow_rate += flow_rates[state.position];
                new_state.pressure += state.flow_rate;

                if memo[new_time][new_state.position][new_state.valves.open_valves]
                    < new_state.pressure
                {
                    memo[new_time][new_state.position][new_state.valves.open_valves] =
                        new_state.pressure;
                    queue[new_time].push(new_state);
                }
            }
        }
    }

    optimal_pressure
}

// Get the shortest paths between all valve pairs using the Floyd-Warshall algorithm
fn get_shortest_paths(valves: &[(&str, usize, Vec<&str>)]) -> Vec<Vec<usize>> {
    let mut shortest_path = vec![vec![usize::MAX / valves.len(); valves.len()]; valves.len()];

    // Set distance to neighbours to 1
    for (valve_index, (_valve, _flow_rate, connected_valves)) in valves.iter().enumerate() {
        shortest_path[valve_index][valve_index] = 0;
        for connected_valve in connected_valves {
            let connected_index = valves
                .iter()
                .position(|(valve, _flow_rate, _connected_valves)| valve == connected_valve)
                .expect("Could not find index of connected valve");
            shortest_path[valve_index][connected_index] = 1;
        }
    }

    // Calculte all-pair shortest paths
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                shortest_path[i][j] = usize::min(
                    shortest_path[i][k] + shortest_path[k][j],
                    shortest_path[i][j],
                );
            }
        }
    }

    shortest_path
}

fn parse_input(line: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    tuple((
        preceded(tag("Valve "), take(2u8)),
        preceded(take_till(|c| is_digit(c as u8)), complete::u32),
        preceded(
            alt((
                pair(take_until("valves "), tag("valves ")),
                pair(take_until("valve "), tag("valve ")),
            )),
            separated_list1(tag(", "), take(2u8)),
        ),
    ))(line)
}

fn parse_data(filename: &str) -> String {
    read_to_string(filename).expect("Could not read datafile")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_single_valve() {
        let line = "Valve HH has flow rate=22; tunnel leads to valve GG";
        let (_res, valve_tuple) = parse_input(line).unwrap();
        assert_eq!(valve_tuple, ("HH", 22, vec!["GG"]));
    }

    #[test]
    fn test_parse_line_multiple_valves() {
        let line = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let (_res, valve_tuple) = parse_input(line).unwrap();
        assert_eq!(valve_tuple, ("AA", 0, vec!["DD", "II", "BB"]));
    }

    #[test]
    fn test_closed_valves() {
        let mut valves = Valves::default();

        let mut closed_valves = valves.get_closed_valves(5);
        assert_eq!(closed_valves.next(), Some(0));
        assert_eq!(closed_valves.next(), Some(1));
        assert_eq!(closed_valves.next(), Some(2));
        assert_eq!(closed_valves.next(), Some(3));
        assert_eq!(closed_valves.next(), Some(4));
        assert_eq!(closed_valves.next(), None);
        drop(closed_valves);

        valves.open_valve(3);
        let mut closed_valves = valves.get_closed_valves(5);
        assert_eq!(closed_valves.next(), Some(0));
        assert_eq!(closed_valves.next(), Some(1));
        assert_eq!(closed_valves.next(), Some(2));
        assert_eq!(closed_valves.next(), Some(4));
        assert_eq!(closed_valves.next(), None);
    }

    #[test]
    fn test_all_valves_open() {
        let mut valves = Valves::default();

        valves.open_valve(0);
        valves.open_valve(1);
        assert!(!valves.all_valves_open(3));

        valves.open_valve(2);
        assert!(valves.all_valves_open(3));
    }

    #[test]
    fn test_part_one() {
        let data = parse_data("data/16-example.txt");
        assert_eq!(part_one(&data), 1651);
    }

    #[test]
    fn test_part_two() {
        let data = parse_data("data/16-example.txt");
        assert_eq!(part_two(&data), 1704);
    }
}
