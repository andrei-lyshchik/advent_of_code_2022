use std::collections::{HashSet, VecDeque};

use aoc2022::input::read_lines;

fn main() {
    let cubes = match read_lines("inputs/day18.txt")
        .map(|l| parse_point(&l))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(points) => points,
        Err(err) => {
            println!("Unable to parse points: {}", err);
            return;
        }
    };
    let open_sides = calculate_open_sides(&cubes);
    println!("Part 1: {}", open_sides.len());
    println!(
        "Part 2: {:?}",
        connected_components_sizes(&open_sides).iter().max()
    );
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn shift(&self, dimension: Dimension, direction: Direction) -> Point {
        let delta = match direction {
            Direction::Up => 1,
            Direction::Down => -1,
        };
        match dimension {
            Dimension::X => Point {
                x: self.x + delta,
                y: self.y,
                z: self.z,
            },
            Dimension::Y => Point {
                x: self.x,
                y: self.y + delta,
                z: self.z,
            },
            Dimension::Z => Point {
                x: self.x,
                y: self.y,
                z: self.z + delta,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dimension {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

fn parse_point(str: &str) -> Result<Point, String> {
    let mut split = str.split(",").map(|p| p.parse());
    match (split.next(), split.next(), split.next()) {
        (Some(Ok(x)), Some(Ok(y)), Some(Ok(z))) => Ok(Point { x, y, z }),
        _ => Err(format!("Unable to parse point out of '{}'", str)),
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct CubeSide {
    center: Point,
    dimension: Dimension,
    direction: Direction,
}

impl CubeSide {
    fn opposite(&self) -> CubeSide {
        CubeSide {
            center: self.center.shift(self.dimension, self.direction),
            dimension: self.dimension,
            direction: self.direction.opposite(),
        }
    }

    fn adjacent_sides(&self, open_sides: &HashSet<CubeSide>) -> Vec<CubeSide> {
        let mut result = vec![];

        let center_of_opposite_cube = self.center.shift(self.dimension, self.direction);
        for dimension in DIMENSIONS.iter().filter(|d| **d != self.dimension) {
            for direction in DIRECTIONS.iter() {
                let side_on_the_same_cube = CubeSide {
                    center: self.center,
                    dimension: *dimension,
                    direction: *direction,
                };
                let shifted_center = self.center.shift(*dimension, *direction);
                let parallel_side = CubeSide {
                    center: shifted_center,
                    dimension: self.dimension,
                    direction: self.direction,
                };
                if open_sides.contains(&parallel_side)
                    && !open_sides.contains(&side_on_the_same_cube)
                    && !open_sides.contains(&side_on_the_same_cube.opposite())
                {
                    result.push(parallel_side);
                }
                if open_sides.contains(&side_on_the_same_cube)
                    && !open_sides.contains(&parallel_side.opposite())
                {
                    result.push(side_on_the_same_cube);
                }
                let side_on_the_adjacent_cube = CubeSide {
                    center: center_of_opposite_cube,
                    dimension: *dimension,
                    direction: *direction,
                }
                .opposite();
                if open_sides.contains(&side_on_the_adjacent_cube) {
                    result.push(side_on_the_adjacent_cube);
                }
            }
        }

        result
    }
}

static DIMENSIONS: [Dimension; 3] = [Dimension::X, Dimension::Y, Dimension::Z];
static DIRECTIONS: [Direction; 2] = [Direction::Down, Direction::Up];

fn calculate_open_sides(centers: &[Point]) -> HashSet<CubeSide> {
    let mut sides = HashSet::new();

    for center in centers {
        for dimension in DIMENSIONS.iter() {
            for direction in DIRECTIONS.iter() {
                let new_side = CubeSide {
                    center: *center,
                    dimension: *dimension,
                    direction: *direction,
                };
                if !sides.remove(&new_side.opposite()) {
                    sides.insert(new_side);
                }
            }
        }
    }

    sides
}

fn connected_components_sizes(open_sides: &HashSet<CubeSide>) -> Vec<usize> {
    let mut not_visited: HashSet<&CubeSide> = HashSet::from_iter(open_sides.iter());
    let mut connected_components_sizes: Vec<usize> = vec![];

    while let Some(connected_component_start) = not_visited.iter().cloned().next() {
        connected_components_sizes.push(0);
        let last_index = connected_components_sizes.len() - 1;
        let mut queue = VecDeque::new();
        queue.push_back(connected_component_start.clone());

        while let Some(side) = queue.pop_front() {
            if !not_visited.contains(&side) {
                continue;
            }
            not_visited.remove(&side);
            connected_components_sizes[last_index] += 1;
            for neighbor in side.adjacent_sides(&open_sides).iter() {
                queue.push_back(neighbor.clone());
            }
        }
    }

    connected_components_sizes
}

#[cfg(test)]
mod tests {
    use std::collections::{hash_map::RandomState, HashSet};

    use crate::{
        calculate_open_sides, connected_components_sizes, parse_point, CubeSide, Dimension,
        Direction, Point,
    };

    #[test]
    fn example_from_description() {
        let cubes = "2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5"
            .lines()
            .map(|l| parse_point(l.trim()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let open_sides = calculate_open_sides(&cubes);
        assert_eq!(vec![58, 6], connected_components_sizes(&open_sides));
    }

    #[test]
    fn test_adjacent_sides() {
        let open_sides = calculate_open_sides(&[
            Point { x: 1, y: 0, z: 0 },
            Point { x: -1, y: 0, z: 0 },
            Point { x: 0, y: 1, z: 0 },
            Point { x: 0, y: -1, z: 0 },
            Point { x: 0, y: 0, z: 1 },
            Point { x: 0, y: 0, z: -1 },
        ]);

        let expected: HashSet<CubeSide, RandomState> = HashSet::from_iter(
            vec![
                CubeSide {
                    center: Point { x: -1, y: 0, z: 0 },
                    dimension: Dimension::X,
                    direction: Direction::Up,
                },
                CubeSide {
                    center: Point { x: 1, y: 0, z: 0 },
                    dimension: Dimension::X,
                    direction: Direction::Down,
                },
                CubeSide {
                    center: Point { x: 0, y: -1, z: 0 },
                    dimension: Dimension::Y,
                    direction: Direction::Up,
                },
                CubeSide {
                    center: Point { x: 0, y: 1, z: 0 },
                    dimension: Dimension::Y,
                    direction: Direction::Down,
                },
            ]
            .iter()
            .cloned(),
        );
        let actual = HashSet::from_iter(
            CubeSide {
                center: Point { x: 0, y: 0, z: 1 },
                dimension: Dimension::Z,
                direction: Direction::Down,
            }
            .adjacent_sides(&open_sides)
            .iter()
            .cloned(),
        );
        assert_eq!(expected, actual);

        let expected: HashSet<CubeSide, RandomState> = HashSet::from_iter(
            vec![
                CubeSide {
                    center: Point { x: 0, y: 0, z: 1 },
                    dimension: Dimension::Y,
                    direction: Direction::Up,
                },
                CubeSide {
                    center: Point { x: 0, y: 0, z: 1 },
                    dimension: Dimension::Y,
                    direction: Direction::Down,
                },
                CubeSide {
                    center: Point { x: 0, y: 0, z: 1 },
                    dimension: Dimension::Z,
                    direction: Direction::Up,
                },
                CubeSide {
                    center: Point { x: 1, y: 0, z: 0 },
                    dimension: Dimension::Z,
                    direction: Direction::Up,
                },
            ]
            .iter()
            .cloned(),
        );
        let actual: HashSet<CubeSide, RandomState> = HashSet::from_iter(
            CubeSide {
                center: Point { x: 0, y: 0, z: 1 },
                dimension: Dimension::X,
                direction: Direction::Up,
            }
            .adjacent_sides(&open_sides)
            .iter()
            .cloned(),
        );
        assert_eq!(expected, actual);
    }
}
