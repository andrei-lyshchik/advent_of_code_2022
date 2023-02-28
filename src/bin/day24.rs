use std::{
    collections::{HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use aoc2022::input::read_lines;

fn main() {
    let map = parse_map(
        read_lines("inputs/day24.txt")
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap();

    println!(
        "Part 1: {:?}",
        map.shortest_path_time_visiting_all(vec![map.goal_coordinates()])
    );
    println!(
        "Part 2: {:?}",
        map.shortest_path_time_visiting_all(vec![
            map.goal_coordinates(),
            map.start_coordinates(),
            map.goal_coordinates()
        ])
    );
}

fn parse_map(lines: Vec<Vec<char>>) -> Result<Map, String> {
    if lines.len() < 3 {
        return Err("Empty list of lines".to_string());
    }
    let width_with_borders = lines[0].len();
    if width_with_borders < 3 {
        return Err("Empty input map".to_string());
    }
    if lines.iter().any(|l| l.len() != width_with_borders) {
        return Err("Non square map".to_string());
    }

    let mut per_row = vec![vec![]; lines.len()];
    let mut per_column = vec![vec![]; width_with_borders - 2];

    for (row_index, row) in lines.iter().enumerate() {
        for (column_index, value) in row.iter().skip(1).enumerate() {
            match value {
                '^' => {
                    per_column[column_index].push(Wind {
                        forward: false,
                        starting_index: row_index - 1,
                    });
                }
                'v' => {
                    per_column[column_index].push(Wind {
                        forward: true,
                        starting_index: row_index - 1,
                    });
                }
                '<' => {
                    per_row[row_index].push(Wind {
                        forward: false,
                        starting_index: column_index,
                    });
                }
                '>' => {
                    per_row[row_index].push(Wind {
                        forward: true,
                        starting_index: column_index,
                    });
                }
                '#' | '.' => {}
                unexpected @ _ => {
                    return Err(format!("Unexpected char: '{}'", unexpected));
                }
            };
        }
    }

    Ok(Map::new(per_row, per_column))
}

#[derive(Debug, Clone)]
struct Wind {
    forward: bool,
    starting_index: usize,
}

impl Wind {
    fn would_be_at(&self, index: usize, time: usize, cycle_time: usize) -> bool {
        if self.forward {
            (self.starting_index + time) % cycle_time == index
        } else {
            (self.starting_index + (cycle_time - (time % cycle_time))) % cycle_time == index
        }
    }
}

#[derive(Debug)]
struct Vec2D {
    width: usize,
    values: Vec<bool>,
}

impl Vec2D {
    fn new(width: usize, height: usize) -> Vec2D {
        Vec2D {
            width,
            values: vec![false; height * width],
        }
    }
}

impl Index<(usize, usize)> for Vec2D {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.values[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Vec2D {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.values[y * self.width + x]
    }
}

#[derive(Debug)]
struct Map {
    possible_moves_at_time: Vec<Vec2D>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(winds_per_row: Vec<Vec<Wind>>, winds_per_column: Vec<Vec<Wind>>) -> Map {
        let horizontal_cycle_time = winds_per_column.len();
        let vertical_cycle_time = winds_per_row.len() - 2;
        let total_cycle_time = least_common_multiple(horizontal_cycle_time, vertical_cycle_time);
        let width = winds_per_column.len();
        let height = winds_per_row.len();
        let possible_moves_at_time = (0..total_cycle_time)
            .map(|time| -> Vec2D {
                let mut possible_moves = Vec2D::new(width, height);
                possible_moves[(0, 0)] = true; // start, the rest of row are walls
                for y in 1..height - 1 {
                    for x in 0..width {
                        let blizzard_from_vertical_wind = winds_per_column[x]
                            .iter()
                            .any(|w| w.would_be_at(y - 1, time, vertical_cycle_time));
                        let blizzard_from_horizontal_wind = winds_per_row[y]
                            .iter()
                            .any(|w| w.would_be_at(x, time, horizontal_cycle_time));
                        possible_moves[(x, y)] =
                            !blizzard_from_vertical_wind && !blizzard_from_horizontal_wind;
                    }
                }
                possible_moves[(width - 1, height - 1)] = true; // finish, the rest of row are walls
                possible_moves
            })
            .collect();

        Map {
            possible_moves_at_time,
            width,
            height,
        }
    }

    fn total_cycle_time(&self) -> usize {
        self.possible_moves_at_time.len()
    }

    fn shortest_path_time_visiting_all(
        &self,
        coordinates_to_visit: Vec<Coordinates>,
    ) -> Option<usize> {
        let mut start = CoordinatesInTime {
            x: 0,
            y: 0,
            time: 0,
        };
        let mut total_time = 0;
        for to_visit in coordinates_to_visit.iter() {
            if let Some(finish) = self.shortest_path_time(&start, to_visit) {
                total_time += finish.time - start.time;
                start = finish;
            } else {
                return None;
            }
        }
        Some(total_time)
    }

    fn shortest_path_time(
        &self,
        start: &CoordinatesInTime,
        finish: &Coordinates,
    ) -> Option<CoordinatesInTime> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(start.clone());
        seen.insert(start.clone());

        while let Some(current_coordinates) = queue.pop_front() {
            for possible_move in self.possible_moves(&current_coordinates) {
                if !seen.insert(self.normalize(&possible_move)) {
                    continue;
                }
                if possible_move.same_as(finish) {
                    return Some(possible_move);
                }
                queue.push_back(possible_move);
            }
        }

        None
    }

    fn normalize(&self, coordinates: &CoordinatesInTime) -> CoordinatesInTime {
        CoordinatesInTime {
            x: coordinates.x,
            y: coordinates.y,
            time: coordinates.time % self.total_cycle_time(),
        }
    }

    fn possible_moves(&self, coordinates: &CoordinatesInTime) -> Vec<CoordinatesInTime> {
        let mut result = vec![];
        if coordinates.y > 0 {
            self.push_if_possible_move(&mut result, coordinates.move_up());
        }
        if coordinates.y < self.height - 1 {
            self.push_if_possible_move(&mut result, coordinates.move_down());
        }
        if coordinates.x > 0 {
            self.push_if_possible_move(&mut result, coordinates.move_left());
        }
        if coordinates.x < self.width - 1 {
            self.push_if_possible_move(&mut result, coordinates.move_right());
        }
        self.push_if_possible_move(&mut result, coordinates.wait());

        result
    }

    fn push_if_possible_move(
        &self,
        vec: &mut Vec<CoordinatesInTime>,
        coordinates: CoordinatesInTime,
    ) {
        if self.is_possible_move(&coordinates) {
            vec.push(coordinates);
        }
    }

    fn is_possible_move(&self, coordinates: &CoordinatesInTime) -> bool {
        self.possible_moves_at_time[coordinates.time % self.total_cycle_time()]
            [(coordinates.x, coordinates.y)]
    }

    fn start_coordinates(&self) -> Coordinates {
        Coordinates { x: 0, y: 0 }
    }

    fn goal_coordinates(&self) -> Coordinates {
        Coordinates {
            x: self.width - 1,
            y: self.height - 1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct CoordinatesInTime {
    x: usize,
    y: usize,
    time: usize,
}

impl CoordinatesInTime {
    fn move_up(&self) -> CoordinatesInTime {
        CoordinatesInTime {
            x: self.x,
            y: self.y - 1,
            time: self.time + 1,
        }
    }

    fn move_down(&self) -> CoordinatesInTime {
        CoordinatesInTime {
            x: self.x,
            y: self.y + 1,
            time: self.time + 1,
        }
    }

    fn move_left(&self) -> CoordinatesInTime {
        CoordinatesInTime {
            x: self.x - 1,
            y: self.y,
            time: self.time + 1,
        }
    }

    fn move_right(&self) -> CoordinatesInTime {
        CoordinatesInTime {
            x: self.x + 1,
            y: self.y,
            time: self.time + 1,
        }
    }

    fn wait(&self) -> CoordinatesInTime {
        CoordinatesInTime {
            x: self.x,
            y: self.y,
            time: self.time + 1,
        }
    }

    fn same_as(&self, coordinates: &Coordinates) -> bool {
        self.x == coordinates.x && self.y == coordinates.y
    }
}

struct Coordinates {
    x: usize,
    y: usize,
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * (b / greatest_common_divisor(a, b))
}

#[cfg(test)]
mod tests {
    use crate::{greatest_common_divisor, least_common_multiple, parse_map, Wind};

    #[test]
    fn example_from_description() {
        let map = parse_map(
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
                .lines()
                .map(|l| l.chars().collect())
                .collect(),
        )
        .unwrap();

        assert_eq!(
            Some(18),
            map.shortest_path_time_visiting_all(vec![map.goal_coordinates()])
        );
    }

    #[test]
    fn test_gcd() {
        assert_eq!(4, greatest_common_divisor(12, 8));
        assert_eq!(4, greatest_common_divisor(8, 12));
        assert_eq!(1, greatest_common_divisor(7, 47));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(24, least_common_multiple(12, 8));
        assert_eq!(700, least_common_multiple(35, 100));
        assert_eq!(700, least_common_multiple(100, 35));
    }

    #[test]
    fn test_wind_would_be_at() {
        assert_eq!(
            true,
            Wind {
                forward: false,
                starting_index: 0
            }
            .would_be_at(7, 1, 8)
        )
    }
}
