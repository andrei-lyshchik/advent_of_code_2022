use std::collections::{HashMap, HashSet};

use aoc2022::input::read_lines;

use Direction::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut elves = parse_initial_coordinates(
        read_lines("inputs/day23.txt")
            .map(|l| l.chars().collect())
            .collect(),
    )
    .unwrap();

    elves.simulate_rounds(10);
    println!(
        "Part 1: {:?}",
        elves.empty_ground_tiles_count_in_smallest_rectangle()
    );
    elves.simulate_rounds_until_nobody_moves();
    println!("Part 2: {:?}", elves.rounds);
}

fn parse_initial_coordinates(lines: Vec<Vec<char>>) -> Result<Elves, String> {
    if lines.is_empty() {
        return Err("Empty elves map".to_string());
    }
    let width = lines[0].len();
    if lines.iter().any(|l| l.len() != width) {
        return Err("Empty map to be a rectangle".to_string());
    }
    let mut elves_coordinates = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match c {
                '#' => {
                    elves_coordinates.insert(Coordinates {
                        x: x as isize,
                        y: y as isize,
                    });
                }
                '.' => {}
                unexpected @ _ => return Err(format!("Unexpected char: '{}'", unexpected)),
            };
        }
    }
    Ok(Elves::new(elves_coordinates))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

lazy_static! {
    static ref DIRECTIONS: [Direction; 4] = [North, South, West, East];
}

impl Direction {
    fn starting_from_index(index: usize) -> [Direction; 4] {
        let mut result = DIRECTIONS.clone();
        result.rotate_left(index);
        result
    }
}

#[derive(Debug)]
struct Elves {
    rounds: usize,
    first_direction_to_propose_index: usize,
    elves_coordinates: HashSet<Coordinates>,
}

impl Elves {
    fn new(elves_coordinates: HashSet<Coordinates>) -> Elves {
        Elves {
            rounds: 0,
            elves_coordinates,
            first_direction_to_propose_index: 0,
        }
    }

    fn simulate_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.simulate_round();
        }
    }

    fn simulate_rounds_until_nobody_moves(&mut self) {
        loop {
            if !self.simulate_round() {
                return;
            }
        }
    }

    fn simulate_round(&mut self) -> bool {
        let mut proposed_moves = HashMap::new();
        for elf_coordinates in self.elves_coordinates.iter() {
            if let Some(proposed_coordinates) = self.propose_move(
                elf_coordinates,
                &Direction::starting_from_index(self.first_direction_to_propose_index),
            ) {
                proposed_moves
                    .entry(proposed_coordinates)
                    .or_insert_with(|| vec![])
                    .push(elf_coordinates.clone());
            }
        }
        let mut any_elf_moved = false;
        for (proposed_coordinates, from_coordinates) in proposed_moves.iter() {
            if from_coordinates.len() == 1 {
                any_elf_moved = true;
                self.elves_coordinates.remove(&from_coordinates[0]);
                self.elves_coordinates.insert(proposed_coordinates.clone());
            }
        }
        self.first_direction_to_propose_index =
            (self.first_direction_to_propose_index + 1) % DIRECTIONS.len();
        self.rounds += 1;
        any_elf_moved
    }

    fn propose_move(
        &self,
        elf_coordinates: &Coordinates,
        directions_order: &[Direction; 4],
    ) -> Option<Coordinates> {
        if (-1..=1).all(|dy| -> bool {
            (-1..=1).all(|dx| -> bool {
                dx == 0 && dy == 0
                    || !self.elves_coordinates.contains(&Coordinates {
                        x: elf_coordinates.x + dx,
                        y: elf_coordinates.y + dy,
                    })
            })
        }) {
            return None;
        }
        for direction in directions_order.iter() {
            let (coordinates_to_check, proposed_coordinates) = match direction {
                North => (
                    [
                        Coordinates {
                            x: elf_coordinates.x - 1,
                            y: elf_coordinates.y - 1,
                        },
                        Coordinates {
                            x: elf_coordinates.x,
                            y: elf_coordinates.y - 1,
                        },
                        Coordinates {
                            x: elf_coordinates.x + 1,
                            y: elf_coordinates.y - 1,
                        },
                    ],
                    Coordinates {
                        x: elf_coordinates.x,
                        y: elf_coordinates.y - 1,
                    },
                ),
                South => (
                    [
                        Coordinates {
                            x: elf_coordinates.x - 1,
                            y: elf_coordinates.y + 1,
                        },
                        Coordinates {
                            x: elf_coordinates.x,
                            y: elf_coordinates.y + 1,
                        },
                        Coordinates {
                            x: elf_coordinates.x + 1,
                            y: elf_coordinates.y + 1,
                        },
                    ],
                    Coordinates {
                        x: elf_coordinates.x,
                        y: elf_coordinates.y + 1,
                    },
                ),
                West => (
                    [
                        Coordinates {
                            x: elf_coordinates.x - 1,
                            y: elf_coordinates.y - 1,
                        },
                        Coordinates {
                            x: elf_coordinates.x - 1,
                            y: elf_coordinates.y,
                        },
                        Coordinates {
                            x: elf_coordinates.x - 1,
                            y: elf_coordinates.y + 1,
                        },
                    ],
                    Coordinates {
                        x: elf_coordinates.x - 1,
                        y: elf_coordinates.y,
                    },
                ),
                East => (
                    [
                        Coordinates {
                            x: elf_coordinates.x + 1,
                            y: elf_coordinates.y - 1,
                        },
                        Coordinates {
                            x: elf_coordinates.x + 1,
                            y: elf_coordinates.y,
                        },
                        Coordinates {
                            x: elf_coordinates.x + 1,
                            y: elf_coordinates.y + 1,
                        },
                    ],
                    Coordinates {
                        x: elf_coordinates.x + 1,
                        y: elf_coordinates.y,
                    },
                ),
            };
            if coordinates_to_check
                .iter()
                .all(|c| !self.elves_coordinates.contains(c))
            {
                return Some(proposed_coordinates);
            }
        }
        return None;
    }

    fn empty_ground_tiles_count_in_smallest_rectangle(&self) -> usize {
        let smallest_rectangle_width = (self.elves_coordinates.iter().map(|c| c.x).max().unwrap()
            - self.elves_coordinates.iter().map(|c| c.x).min().unwrap()
            + 1) as usize;
        let smallest_rectangle_height = (self.elves_coordinates.iter().map(|c| c.y).max().unwrap()
            - self.elves_coordinates.iter().map(|c| c.y).min().unwrap()
            + 1) as usize;

        smallest_rectangle_width * smallest_rectangle_height - self.elves_coordinates.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_initial_coordinates;

    #[test]
    fn example_from_description() {
        let mut elves = parse_initial_coordinates(
            "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
.............."
                .lines()
                .map(|l| l.chars().collect())
                .collect(),
        )
        .unwrap();
        elves.simulate_rounds(10);
        assert_eq!(110, elves.empty_ground_tiles_count_in_smallest_rectangle());
        elves.simulate_rounds_until_nobody_moves();
        assert_eq!(20, elves.rounds);
    }
}
