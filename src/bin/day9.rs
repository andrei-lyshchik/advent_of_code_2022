use std::collections::HashSet;

use aoc2022::input::read_lines;

#[macro_use]
extern crate lazy_static;

fn main() {
    let motions: Vec<_> = read_lines("inputs/day9.txt")
        .map(|l| Motion::parse(&l).unwrap())
        .collect();

    let visited_positions = calculate_visited_positions_by_tail(0, &motions);

    println!(
        "Part 1: number of visited positions by tail, 2 knots: {}",
        visited_positions.len(),
    );

    let visited_positions = calculate_visited_positions_by_tail(8, &motions);

    println!(
        "Part 2: number of visited positions by tail, 10 knots: {}",
        visited_positions.len(),
    );
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Motion {
    direction: Direction,
    distance: usize,
}

impl Motion {
    fn parse(line: &str) -> Result<Motion, String> {
        if line.len() < 3 {
            return Err("Can't parse motion".to_owned());
        }

        if let Some((direction_part, distance_part)) = line.split_once(' ') {
            let direction = match direction_part {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => {
                    return Err(format!(
                        "Can't parse motion direction out of '{}'",
                        direction_part
                    ))
                }
            };

            let distance = match distance_part.parse() {
                Ok(distance) => distance,
                Err(err) => return Err(format!("Can't parse motion distance: {}", err)),
            };

            Ok(Motion {
                direction,
                distance,
            })
        } else {
            Err(format!("Can't parse motion out of '{}'", line))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

lazy_static! {
    static ref START: Position = Position { x: 0, y: 0 };
}

impl Position {
    fn move_in_direction(self: &Position, direction: &Direction) -> Position {
        match direction {
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

fn calculate_visited_positions_by_tail(
    intermediate_knots: usize,
    motions: &Vec<Motion>,
) -> HashSet<Position> {
    let mut current_positions = vec![*START; intermediate_knots + 2];
    let mut tail_visited = HashSet::from([*START]);

    for motion in motions {
        for _ in 0..motion.distance {
            current_positions[0] = current_positions[0].move_in_direction(&motion.direction);
            for i in 1..current_positions.len() {
                if let Some(new_position) =
                    move_back_knot(current_positions[i - 1], current_positions[i])
                {
                    current_positions[i] = new_position;
                    if i == current_positions.len() - 1 {
                        tail_visited.insert(new_position);
                    }
                }
            }
        }
    }

    tail_visited
}

fn move_back_knot(forward: Position, back: Position) -> Option<Position> {
    let delta_x = forward.x - back.x;
    let delta_y = forward.y - back.y;
    if delta_x.abs() <= 1 && delta_y.abs() <= 1 {
        return None;
    }

    if delta_x == 0 || delta_y == 0 {
        return Some(Position {
            x: back.x + delta_x.signum(),
            y: back.y + delta_y.signum(),
        });
    }

    if delta_x.abs() > delta_y.abs() {
        Some(Position {
            x: back.x + delta_x.signum(),
            y: forward.y,
        })
    } else if delta_x.abs() < delta_y.abs() {
        Some(Position {
            x: forward.x,
            y: back.y + delta_y.signum(),
        })
    } else {
        Some(Position {
            x: back.x + delta_x.signum(),
            y: back.y + delta_y.signum(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{calculate_visited_positions_by_tail, Direction, Motion, Position, START};

    #[test]
    fn test_start_is_visited_at_the_beginning() {
        let result = calculate_visited_positions_by_tail(0, &vec![]);

        assert_eq!(HashSet::from([*START]), result);
    }

    #[test]
    fn test_single_motion_only_head_moves() {
        let result = calculate_visited_positions_by_tail(
            0,
            &vec![Motion {
                direction: Direction::Right,
                distance: 1,
            }],
        );

        assert_eq!(HashSet::from([*START]), result);
    }

    #[test]
    fn test_single_motion_head_and_tail_moves_same_direction() {
        let result = calculate_visited_positions_by_tail(
            0,
            &vec![Motion {
                direction: Direction::Up,
                distance: 2,
            }],
        );

        assert_eq!(HashSet::from([*START, Position { x: 0, y: 1 }]), result,);

        let result = calculate_visited_positions_by_tail(
            0,
            &vec![Motion {
                direction: Direction::Left,
                distance: 2,
            }],
        );

        assert_eq!(HashSet::from([*START, Position { x: -1, y: 0 }]), result);
    }

    #[test]
    fn test_aligning_diagonal_tail_move() {
        let result = calculate_visited_positions_by_tail(
            0,
            &vec![
                Motion {
                    direction: Direction::Up,
                    distance: 1,
                },
                Motion {
                    direction: Direction::Right,
                    distance: 1,
                },
                Motion {
                    direction: Direction::Up,
                    distance: 1,
                },
            ],
        );

        assert_eq!(HashSet::from([*START, Position { x: 1, y: 1 }]), result);

        let result = calculate_visited_positions_by_tail(
            0,
            &vec![
                Motion {
                    direction: Direction::Up,
                    distance: 1,
                },
                Motion {
                    direction: Direction::Right,
                    distance: 2,
                },
            ],
        );

        assert_eq!(HashSet::from([*START, Position { x: 1, y: 1 }]), result)
    }

    #[test]
    fn test_diagonal_tail_move() {
        let result = calculate_visited_positions_by_tail(
            4,
            &vec![
                Motion {
                    direction: Direction::Right,
                    distance: 4,
                },
                Motion {
                    direction: Direction::Up,
                    distance: 4,
                },
            ],
        );

        assert_eq!(HashSet::from([*START, Position { x: 1, y: 1 }]), result);
    }

    #[test]
    fn test_medium_example_with_intermediate_knots() {
        let result = calculate_visited_positions_by_tail(
            8,
            &vec![
                Motion {
                    direction: Direction::Right,
                    distance: 4,
                },
                Motion {
                    direction: Direction::Up,
                    distance: 4,
                },
                Motion {
                    direction: Direction::Left,
                    distance: 3,
                },
                Motion {
                    direction: Direction::Down,
                    distance: 1,
                },
                Motion {
                    direction: Direction::Right,
                    distance: 4,
                },
                Motion {
                    direction: Direction::Down,
                    distance: 1,
                },
                Motion {
                    direction: Direction::Left,
                    distance: 5,
                },
                Motion {
                    direction: Direction::Right,
                    distance: 2,
                },
            ],
        );

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_large_example_with_intermediate_knots() {
        let result = calculate_visited_positions_by_tail(
            8,
            &vec![
                Motion {
                    direction: Direction::Right,
                    distance: 5,
                },
                Motion {
                    direction: Direction::Up,
                    distance: 8,
                },
                Motion {
                    direction: Direction::Left,
                    distance: 8,
                },
                Motion {
                    direction: Direction::Down,
                    distance: 3,
                },
                Motion {
                    direction: Direction::Right,
                    distance: 17,
                },
                Motion {
                    direction: Direction::Down,
                    distance: 10,
                },
                Motion {
                    direction: Direction::Left,
                    distance: 25,
                },
                Motion {
                    direction: Direction::Up,
                    distance: 20,
                },
            ],
        );

        assert_eq!(result.len(), 36);
    }
}
