use std::collections::HashMap;

use aoc2022::input::read_lines;
use itertools::Itertools;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn main() {
    let lines = read_lines("inputs/day16.txt").collect::<Vec<_>>();
    let graph = match parse_graph(&lines) {
        Ok(graph) => graph,
        Err(err) => {
            println!("Can't parse graph: {:?}", err);
            return;
        }
    };
    println!("Part 1: {:?}", max_pressure_to_release(&graph, "AA", 30));
    println!(
        "Part 2: {:?}",
        max_pressure_to_release_with_elephant(&graph, "AA", 26)
    );
}

#[derive(Debug)]
struct Graph<'a> {
    neighbors: HashMap<&'a str, Vec<&'a str>>,
    flow_rates: HashMap<&'a str, usize>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Room<'a> {
    name: &'a str,
    flow_rate: usize,
}

impl<'a> Graph<'a> {
    fn shortest_paths(self: &Graph<'a>) -> HashMap<(&str, &str), usize> {
        let mut result = HashMap::new();
        for (room, neighbors) in self.neighbors.iter() {
            for neighbor in neighbors.iter() {
                result.insert((*room, *neighbor), 1);
            }
            result.insert((room, room), 0);
        }
        for room1 in self.neighbors.keys() {
            for room2 in self.neighbors.keys() {
                for room3 in self.neighbors.keys() {
                    if let Some(dist_from_2_to_1) = result.get(&(room2, room1)) {
                        if let Some(dist_from_1_to_3) = result.get(&(room1, room3)) {
                            if let Some(dist_from_2_to_3) = result.get(&(room2, room3)) {
                                if *dist_from_2_to_3 > dist_from_2_to_1 + dist_from_1_to_3 {
                                    result.insert(
                                        (room2, room3),
                                        dist_from_2_to_1 + dist_from_1_to_3,
                                    );
                                }
                            } else {
                                result.insert((room2, room3), dist_from_2_to_1 + dist_from_1_to_3);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    fn rooms_with_positive_flow_rates(self: &Self) -> Vec<Room<'a>> {
        self.flow_rates
            .iter()
            .filter_map(|(room, flow_rate)| {
                if *flow_rate > 0 {
                    Some(Room {
                        name: *room,
                        flow_rate: *flow_rate,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

fn parse_graph<'a>(lines: &'a Vec<String>) -> Result<Graph<'a>, String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(, [A-Z]{2})*)"
        )
        .unwrap();
    }

    let mut edges = HashMap::new();
    let mut flow_rates = HashMap::new();
    for line in lines {
        if let Some(captures) = LINE_RE.captures(line) {
            let room = captures.get(1).unwrap().as_str();
            let flow_rate = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .map_err(|e| format!("Can't parse flow rate: {}", e))?;
            flow_rates.insert(room, flow_rate);
            let neighbor_rooms = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .collect::<Vec<_>>();
            edges.insert(room, neighbor_rooms);
        } else {
            return Err(format!("Can't parse vertex info from '{}'", line));
        }
    }

    Ok(Graph {
        neighbors: edges,
        flow_rates,
    })
}

fn max_pressure_to_release(graph: &Graph, start_at_room: &str, minutes: usize) -> usize {
    let shortest_paths = graph.shortest_paths();
    let rooms_with_positive_flow_rates = graph.rooms_with_positive_flow_rates();
    let mut solved = HashMap::new();

    max_pressure_to_release_recursive(
        &Task {
            start_at_room,
            unopened_rooms: rooms_with_positive_flow_rates,
            minutes,
        },
        &mut solved,
        &shortest_paths,
    )
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Task<'a> {
    start_at_room: &'a str,
    unopened_rooms: Vec<Room<'a>>,
    minutes: usize,
}

fn max_pressure_to_release_recursive<'a>(
    task: &Task<'a>,
    solved: &mut HashMap<Task<'a>, usize>,
    shortest_paths: &HashMap<(&str, &str), usize>,
) -> usize {
    task.unopened_rooms
        .iter()
        .filter_map(|unopened_room| {
            shortest_paths
                .get(&(task.start_at_room, unopened_room.name))
                .map(|distance_to_travel| (unopened_room, distance_to_travel + 1))
        })
        .filter(|(_, distance_to_travel_and_open)| *distance_to_travel_and_open < task.minutes)
        .map(|(unopened_room, distance_to_travel_and_open)| {
            let subtask_minutes = task.minutes - distance_to_travel_and_open;
            let subtask = Task {
                start_at_room: unopened_room.name,
                unopened_rooms: open_room(&task.unopened_rooms, unopened_room.name),
                minutes: subtask_minutes,
            };
            let subtask_solution = if let Some(already_solved) = solved.get(&subtask) {
                *already_solved
            } else {
                let solution = max_pressure_to_release_recursive(&subtask, solved, shortest_paths);
                solved.insert(subtask, solution);
                solution
            };
            subtask_minutes * unopened_room.flow_rate + subtask_solution
        })
        .max()
        .unwrap_or(0)
}

fn open_room<'a>(unopened_rooms: &Vec<Room<'a>>, room_name: &str) -> Vec<Room<'a>> {
    unopened_rooms
        .iter()
        .cloned()
        .filter(|ur| ur.name != room_name)
        .collect()
}

fn max_pressure_to_release_with_elephant(
    graph: &Graph,
    start_at_room: &str,
    minutes: usize,
) -> usize {
    let shortest_paths = graph.shortest_paths();
    let rooms_with_positive_flow_rates = graph.rooms_with_positive_flow_rates();
    let mut solved = HashMap::new();

    (1..=rooms_with_positive_flow_rates.len() / 2)
        .flat_map(|rooms_count_to_open_for_you| {
            rooms_with_positive_flow_rates
                .iter()
                .cloned()
                .combinations(rooms_count_to_open_for_you)
        })
        .map(|rooms_to_open_for_you| {
            let rooms_to_open_for_elephant = rooms_with_positive_flow_rates
                .iter()
                .cloned()
                .filter(|r| !rooms_to_open_for_you.contains(r))
                .collect::<Vec<_>>();

            let pressure_released_by_you = max_pressure_to_release_recursive(
                &Task {
                    start_at_room,
                    unopened_rooms: rooms_to_open_for_you,
                    minutes,
                },
                &mut solved,
                &shortest_paths,
            );
            let pressure_released_by_elephant = max_pressure_to_release_recursive(
                &Task {
                    start_at_room,
                    unopened_rooms: rooms_to_open_for_elephant,
                    minutes,
                },
                &mut solved,
                &shortest_paths,
            );
            pressure_released_by_you + pressure_released_by_elephant
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::{max_pressure_to_release, parse_graph};

    #[test]
    fn example_from_description_part1() {
        let lines = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II"
            .lines()
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>();
        let graph = parse_graph(&lines).unwrap();

        assert_eq!(1651, max_pressure_to_release(&graph, "AA", 30));
    }
}
