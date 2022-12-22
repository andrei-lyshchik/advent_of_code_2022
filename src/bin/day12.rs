use std::collections::{HashMap, HashSet, VecDeque};

use aoc2022::input::read_lines;

fn main() {
    let graph = match parse_graph(
        &read_lines("inputs/day12.txt")
            .map(|l| l.into_bytes())
            .collect(),
    ) {
        Ok(graph) => graph,
        Err(err) => {
            println!("Couldn't parse graph: {}", err);
            return;
        }
    };
    println!(
        "Part 1: shortest path length {:?}",
        shortest_path_length(&graph, graph.part1_start)
    );
    println!(
        "Part 2: shortest path length from 'a' coord: {:?}",
        graph
            .a_coords
            .iter()
            .flat_map(|c| shortest_path_length(&graph, *c))
            .min()
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Graph {
    part1_start: Coordinate,
    a_coords: Vec<Coordinate>,
    end: Coordinate,
    edges: HashMap<Coordinate, Vec<Coordinate>>,
}

fn parse_graph(lines: &Vec<Vec<u8>>) -> Result<Graph, String> {
    if lines.is_empty() {
        return Err("Can't parse graph out of empty lines".to_owned());
    }
    let common_len = lines[0].len();
    if lines.iter().any(|l| l.len() != common_len) {
        return Err("Expected all lines to have the same length".to_owned());
    }

    let mut part1_start = None;
    let mut end = None;
    let mut edges = HashMap::new();
    let mut a_coords = Vec::new();
    for y in 0..lines.len() {
        let dy_bounds = if y == 0 { 0..=1 } else { -1i32..=1 };
        for x in 0..lines[y].len() {
            let dx_bounds = if x == 0 { 0..=1 } else { -1i32..=1 };
            let coordinate = Coordinate { x, y };
            match lines[y][x] {
                b'S' => {
                    part1_start = Some(coordinate);
                }
                b'E' => {
                    end = Some(coordinate);
                }
                b'a'..=b'z' => {}
                unexpected @ _ => return Err(format!("Unexpected char {}", unexpected)),
            };
            let height = get_height(lines[y][x]);
            if height == b'a' {
                a_coords.push(coordinate);
            }
            for dy in dy_bounds.clone() {
                for dx in dx_bounds.clone() {
                    if dy == 0 && dx == 0 || dy != 0 && dx != 0 {
                        continue;
                    }
                    let neighbor_coordinate = Coordinate {
                        x: (x as i32 + dx) as usize,
                        y: (y as i32 + dy) as usize,
                    };
                    if let Some(neighbor_line) = lines.get(neighbor_coordinate.y) {
                        if let Some(neighbor_value) = neighbor_line.get(neighbor_coordinate.x) {
                            if height + 1 >= get_height(*neighbor_value) {
                                edges
                                    .entry(coordinate)
                                    .or_insert_with(|| vec![])
                                    .push(neighbor_coordinate);
                            }
                        }
                    }
                }
            }
        }
    }
    match (part1_start, end) {
        (Some(part1_start), Some(end)) => Ok(Graph {
            part1_start,
            end,
            edges,
            a_coords,
        }),
        _ => Err("Graph didn't have start/end".to_owned()),
    }
}

fn get_height(value: u8) -> u8 {
    match value {
        b'S' => b'a',
        b'E' => b'z',
        other @ _ => other,
    }
}

fn shortest_path_length(graph: &Graph, start: Coordinate) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut visited = HashSet::from([start]);
    let mut parents = HashMap::new();
    while let Some(vertice) = queue.pop_front() {
        for neighbor in graph.edges[&vertice].iter() {
            if visited.contains(neighbor) {
                continue;
            }
            parents.insert(*neighbor, vertice);
            if *neighbor == graph.end {
                break;
            }
            visited.insert(*neighbor);
            queue.push_back(*neighbor);
        }
    }
    let mut current = graph.end;
    let mut path_length = 1;
    while let Some(parent) = parents.get(&current) {
        if *parent == start {
            return Some(path_length);
        }
        path_length += 1;
        current = *parent;
    }
    None
}
