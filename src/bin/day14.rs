use std::{collections::HashSet, ops::RangeInclusive};

use aoc2022::input::read_lines;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SAND_SOURCE: Coordinate = Coordinate { x: 500, y: 0 };
}

fn main() {
    let rocks_turns = match read_lines("inputs/day14.txt")
        .map(|l| parse_rock_turns(&l))
        .collect::<Result<Vec<_>, String>>()
    {
        Ok(coordinates) => coordinates,
        Err(err) => {
            println!("Couldn't parse coordinates: {}", err);
            return;
        }
    };

    let map = match Map::new(&rocks_turns) {
        Ok(map) => map,
        Err(err) => {
            println!("Couldn't parse map: {}", err);
            return;
        }
    };

    println!("Part 1: {}", map.sand_units_until_escape_from_rocks());
    println!("Part 2: {}", map.sand_units_until_source_blocked());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn down(self: &Self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_and_left(self: &Self) -> Coordinate {
        Coordinate {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_and_right(self: &Self) -> Coordinate {
        Coordinate {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn parse_rock_turns(line: &str) -> Result<Vec<Coordinate>, String> {
    let mut result = Vec::new();

    for coordinate_str in line.split(" -> ") {
        let mut coordinate_split = coordinate_str.split(',');
        let coordinate = match (
            coordinate_split.next().map(|s| s.parse()),
            coordinate_split.next().map(|s| s.parse()),
        ) {
            (Some(Ok(x)), Some(Ok(y))) => Coordinate { x, y },
            _ => return Err(format!("Couldn't parse coordinate from {}", coordinate_str)),
        };
        result.push(coordinate);
    }

    Ok(result)
}

#[derive(Debug)]
struct Map {
    rocks_coordinates: HashSet<Coordinate>,
    rocks_x_range: RangeInclusive<i32>,
    floor: i32,
}

impl Map {
    fn new(rocks_turns: &Vec<Vec<Coordinate>>) -> Result<Map, String> {
        if rocks_turns.is_empty() || rocks_turns[0].is_empty() {
            return Err("Can't parse map without rocks".to_owned());
        }
        let mut rocks = HashSet::new();
        let mut rocks_x_min = rocks_turns[0][0].x;
        let mut rocks_x_max = rocks_turns[0][0].x;
        let mut rocks_y_max = rocks_turns[0][0].y;

        for rock_turns in rocks_turns {
            if rock_turns.len() < 2 {
                return Err("There must be at least 2 turns".to_owned());
            }
            for line in rock_turns.windows(2) {
                let line_x_min = line[0].x.min(line[1].x);
                let line_x_max = line[0].x.max(line[1].x);
                rocks_x_min = rocks_x_min.min(line_x_min);
                rocks_x_max = rocks_x_max.max(line_x_max);
                let line_y_min = line[0].y.min(line[1].y);
                let line_y_max = line[0].y.max(line[1].y);
                rocks_y_max = rocks_y_max.max(line_y_max);
                if line[0].x == line[1].x {
                    for y in line_y_min..=line_y_max {
                        rocks.insert(Coordinate { x: line[0].x, y });
                    }
                } else if line[0].y == line[1].y {
                    for x in line_x_min..=line_x_max {
                        rocks.insert(Coordinate { x, y: line[0].y });
                    }
                } else {
                    return Err(format!(
                        "Illegal rock turn sequence: from {:?} to {:?}",
                        line[0], line[1]
                    ));
                }
            }
        }

        Ok(Map {
            rocks_coordinates: rocks,
            rocks_x_range: rocks_x_min..=rocks_x_max,
            floor: rocks_y_max + 2,
        })
    }

    fn sand_units_until_escape_from_rocks(self: &Self) -> usize {
        let mut rest_sand: HashSet<Coordinate> = HashSet::new();

        loop {
            let mut current = SAND_SOURCE.clone();
            loop {
                let mut next = None;
                for next_try in &[
                    current.down(),
                    current.down_and_left(),
                    current.down_and_right(),
                ] {
                    if self.escaped_from_rocks(next_try) {
                        return rest_sand.len();
                    }
                    if !self.blocked(&rest_sand, next_try) {
                        next = Some(*next_try);
                        break;
                    }
                }
                if let Some(next) = next {
                    current = next;
                } else {
                    rest_sand.insert(current);
                    break;
                }
            }
        }
    }

    fn escaped_from_rocks(self: &Self, coordinate: &Coordinate) -> bool {
        !self.rocks_x_range.contains(&coordinate.x)
    }

    fn blocked(self: &Self, rest_sand: &HashSet<Coordinate>, coordinate: &Coordinate) -> bool {
        self.rocks_coordinates.contains(coordinate)
            || rest_sand.contains(coordinate)
            || coordinate.y >= self.floor
    }

    fn sand_units_until_source_blocked(self: &Self) -> usize {
        let mut rest_sand: HashSet<Coordinate> = HashSet::new();

        while !self.blocked(&rest_sand, &*SAND_SOURCE) {
            let mut current = SAND_SOURCE.clone();
            loop {
                let mut next = None;
                for next_try in &[
                    current.down(),
                    current.down_and_left(),
                    current.down_and_right(),
                ] {
                    if !self.blocked(&rest_sand, next_try) {
                        next = Some(*next_try);
                        break;
                    }
                }
                if let Some(next) = next {
                    current = next;
                } else {
                    rest_sand.insert(current);
                    break;
                }
            }
        }
        rest_sand.len()
    }
}
