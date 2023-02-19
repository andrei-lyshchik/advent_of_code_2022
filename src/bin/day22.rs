use std::collections::{HashMap, HashSet, VecDeque};

use aoc2022::input::read_lines;
use DieSide::*;
use Direction::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    let map = match parse_map(read_lines("inputs/day22.txt").collect()) {
        Ok(map) => map,
        Err(err) => {
            println!("Can't parse map and instructions: {}", err);
            return;
        }
    };

    println!("Part 1: {:?}", map.final_password(WrapAroundMethod::In2d));
    println!(
        "Part 2: {:?}",
        map.final_password(WrapAroundMethod::In3dCube)
    );
}

fn parse_map(lines: Vec<String>) -> Result<Map, String> {
    let map_lines = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    if map_lines.is_empty() || lines.len() != map_lines.len() + 2 {
        return Err("Unexpected number of lines".to_string());
    }

    let map_width = map_lines.iter().map(|l| l.len()).max().unwrap();
    if map_width == 0 {
        return Err("Map width is 0".to_string());
    }

    let square_size = ((map_lines
        .iter()
        .map(|l| l.iter().filter(|c| **c != ' ').count())
        .sum::<usize>()
        / 6) as f64)
        .sqrt() as usize;

    let width_in_squares = map_width / square_size;
    let height_in_squares = map_lines.len() / square_size;
    let mut squares = vec![vec![None; width_in_squares]; height_in_squares];

    for square_y in 0..height_in_squares {
        for square_x in 0..width_in_squares {
            squares[square_y][square_x] = parse_square(
                &map_lines,
                Coordinates {
                    x: square_x,
                    y: square_y,
                },
                square_size,
            )?;
        }
    }

    Map::new(squares, parse_instructions(&lines[lines.len() - 1])?)
}

fn parse_square(
    map_lines: &Vec<Vec<char>>,
    square_coordinates: Coordinates,
    square_size: usize,
) -> Result<Option<Square>, String> {
    let top_left_tile = Coordinates {
        x: square_size * square_coordinates.x,
        y: square_size * square_coordinates.y,
    };
    let down_right_tile = Coordinates {
        x: square_size * square_coordinates.x + square_size - 1,
        y: square_size * square_coordinates.y + square_size - 1,
    };
    let mut tile_rows = vec![];
    for tile_y in top_left_tile.y..=down_right_tile.y {
        let mut row = vec![];
        for tile_x in top_left_tile.x..=down_right_tile.x {
            let tile = match map_lines[tile_y].get(tile_x) {
                Some('.') => Tile::Open,
                Some('#') => Tile::Wall,
                Some(' ') => return Ok(None),
                None => return Ok(None),
                invalid_char @ _ => {
                    return Err(format!("Invalid character in map: {:?}", invalid_char))
                }
            };
            row.push(tile);
        }
        tile_rows.push(row);
    }
    Ok(Some(Square { tile_rows }))
}

fn parse_instructions(line: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = vec![];
    let mut number_start = None;
    for (i, char) in line.char_indices() {
        if char.is_numeric() {
            if number_start.is_none() {
                number_start = Some(i);
            }
        } else {
            if let Some(number_start) = number_start {
                instructions.push(Instruction::Go(
                    line[number_start..i]
                        .parse()
                        .map_err(|e| format!("Can't parse go count: {}", e))?,
                ));
            }
            number_start = None;
            match char {
                'R' => instructions.push(Instruction::TurnClockwise),
                'L' => instructions.push(Instruction::TurnCounterClockwise),
                _ => return Err(format!("Unexpected char in instructions line: {}", char)),
            }
        }
    }
    if let Some(number_start) = number_start {
        instructions.push(Instruction::Go(
            line[number_start..line.len()]
                .parse()
                .map_err(|e| format!("Can't parse go count: {}", e))?,
        ));
    }
    Ok(instructions)
}

#[derive(Debug)]
struct Map {
    squares: Vec<Vec<Option<Square>>>,
    adjacent_edges: HashMap<SquareEdge, SquareEdge>,
    instructions: Vec<Instruction>,
    square_size: usize,
}

impl Map {
    fn new(
        squares: Vec<Vec<Option<Square>>>,
        instructions: Vec<Instruction>,
    ) -> Result<Map, String> {
        if squares.is_empty() || squares[0].is_empty() {
            return Err("Empty map".to_string());
        }
        let width = squares[0].len();
        for square_row in squares.iter() {
            if square_row.len() != width {
                return Err(format!(
                    "Wrong square row width: {}, expected all rows to have width {}",
                    square_row.len(),
                    width
                ));
            }
        }
        if let Some(starting_square_coordinates) = Self::starting_square_coordinates(&squares) {
            let adjacent_edges =
                Self::calculate_adjacent_edges(&squares, starting_square_coordinates)?;
            let square_size = squares[starting_square_coordinates.y][starting_square_coordinates.x]
                .as_ref()
                .unwrap()
                .tile_rows
                .len();
            Ok(Map {
                squares,
                adjacent_edges,
                instructions,
                square_size,
            })
        } else {
            Err("No square in first row".to_string())
        }
    }

    fn starting_square_coordinates(squares: &Vec<Vec<Option<Square>>>) -> Option<Coordinates> {
        squares[0]
            .iter()
            .enumerate()
            .find(|(_, s)| s.is_some())
            .map(|(i, _)| i)
            .map(|x| Coordinates { x, y: 0 })
    }

    fn calculate_adjacent_edges(
        squares: &Vec<Vec<Option<Square>>>,
        starting_square_coordinates: Coordinates,
    ) -> Result<HashMap<SquareEdge, SquareEdge>, String> {
        let mut from_square_to_die: HashMap<SquareEdge, DieEdge> = HashMap::new();
        let mut from_die_to_square: HashMap<DieEdge, SquareEdge> = HashMap::new();
        let mut assigned_squares: HashSet<Coordinates> = HashSet::new();

        Self::assign(
            SquareEdge {
                coordinates: starting_square_coordinates,
                direction: Up,
            },
            DieEdge {
                side: One,
                direction: Up,
            },
            &mut from_square_to_die,
            &mut from_die_to_square,
            &mut assigned_squares,
        );
        let mut queue = VecDeque::new();
        queue.push_back(starting_square_coordinates);

        while let Some(coordinates) = queue.pop_front() {
            for direction in DIRECTIONS.iter().cloned() {
                let square_edge = SquareEdge {
                    coordinates,
                    direction,
                };
                if let Some(adjacent_square_edge) =
                    Self::adjacent_based_on_2d_map(square_edge, &squares)
                {
                    if assigned_squares.contains(&adjacent_square_edge.coordinates) {
                        continue;
                    }
                    let adjacent_die_edge = *DIE_EDGES
                        .get(from_square_to_die.get(&square_edge).unwrap())
                        .unwrap();
                    Self::assign(
                        adjacent_square_edge,
                        adjacent_die_edge,
                        &mut from_square_to_die,
                        &mut from_die_to_square,
                        &mut assigned_squares,
                    );
                    queue.push_back(adjacent_square_edge.coordinates);
                }
            }
        }
        let mut result = HashMap::new();
        for y in 0..squares.len() {
            for x in 0..squares[0].len() {
                if squares[y][x].is_none() {
                    continue;
                }
                let coordinates = Coordinates { x, y };
                for direction in DIRECTIONS.iter().cloned() {
                    let square_edge = SquareEdge {
                        coordinates,
                        direction,
                    };
                    let adjacent_square_edge = from_square_to_die
                        .get(&square_edge)
                        .and_then(|de| DIE_EDGES.get(de))
                        .and_then(|ade| from_die_to_square.get(ade))
                        .cloned()
                        .ok_or_else(|| {
                            format!("Couldn't find adjacent edge for {:?}", square_edge)
                        })?;
                    result.insert(square_edge, adjacent_square_edge);
                }
            }
        }
        Ok(result)
    }

    fn assign(
        square_edge: SquareEdge,
        die_edge: DieEdge,
        from_square_to_die: &mut HashMap<SquareEdge, DieEdge>,
        from_die_to_square: &mut HashMap<DieEdge, SquareEdge>,
        assigned_squares: &mut HashSet<Coordinates>,
    ) {
        for (square_edge_direction, die_edge_direction) in
            directions_starting_from(square_edge.direction)
                .iter()
                .zip(directions_starting_from(die_edge.direction))
        {
            let matching_square_edge = SquareEdge {
                coordinates: square_edge.coordinates,
                direction: *square_edge_direction,
            };
            let matching_die_edge = DieEdge {
                side: die_edge.side,
                direction: die_edge_direction,
            };
            from_square_to_die.insert(matching_square_edge, matching_die_edge);
            from_die_to_square.insert(matching_die_edge, matching_square_edge);
        }
        assigned_squares.insert(square_edge.coordinates);
    }

    fn adjacent_based_on_2d_map(
        square_edge: SquareEdge,
        squares: &Vec<Vec<Option<Square>>>,
    ) -> Option<SquareEdge> {
        square_edge
            .coordinates
            .shift(square_edge.direction, squares[0].len(), squares.len())
            .and_then(|adjacent_coordinates| -> Option<SquareEdge> {
                if squares[adjacent_coordinates.y][adjacent_coordinates.x].is_none() {
                    return None;
                }
                Some(SquareEdge {
                    coordinates: adjacent_coordinates,
                    direction: square_edge.direction.opposite(),
                })
            })
    }

    fn final_password(&self, wrap_around_method: WrapAroundMethod) -> usize {
        let final_position = self.position_after_following_instructions(wrap_around_method);
        self.final_password_for_position(&final_position)
    }

    fn position_after_following_instructions(
        &self,
        wrap_around_method: WrapAroundMethod,
    ) -> MapPosition {
        let mut current_position = MapPosition {
            square_coordinates: Self::starting_square_coordinates(&self.squares).unwrap(),
            tile_coordinates: Coordinates { x: 0, y: 0 },
            direction: Right,
        };

        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Go(count) => {
                    for _ in 0..*count {
                        let next_position = self.shift(&current_position, wrap_around_method);
                        match self.tile_at(next_position) {
                            Tile::Open => {
                                current_position = next_position;
                            }
                            Tile::Wall => break,
                        }
                    }
                }
                Instruction::TurnClockwise => {
                    current_position.direction = current_position.direction.turn_clockwise();
                }
                Instruction::TurnCounterClockwise => {
                    current_position.direction =
                        current_position.direction.turn_counter_clockwise();
                }
            }
        }

        current_position
    }

    fn final_password_for_position(&self, position: &MapPosition) -> usize {
        let global_row =
            position.square_coordinates.y * self.square_size + position.tile_coordinates.y + 1;
        let global_column =
            position.square_coordinates.x * self.square_size + position.tile_coordinates.x + 1;
        1000 * global_row
            + 4 * global_column
            + match position.direction {
                Direction::Up => 3,
                Direction::Right => 0,
                Direction::Down => 1,
                Direction::Left => 2,
            }
    }

    fn tile_at(&self, map_coordinates: MapPosition) -> Tile {
        let square = self.squares[map_coordinates.square_coordinates.y]
            [map_coordinates.square_coordinates.x]
            .as_ref()
            .unwrap();
        square.tile_rows[map_coordinates.tile_coordinates.y][map_coordinates.tile_coordinates.x]
    }

    fn shift(&self, position: &MapPosition, wrap_around_method: WrapAroundMethod) -> MapPosition {
        if let Some(next_tile_coordinates_inside_same_square) =
            position
                .tile_coordinates
                .shift(position.direction, self.square_size, self.square_size)
        {
            MapPosition {
                square_coordinates: position.square_coordinates,
                tile_coordinates: next_tile_coordinates_inside_same_square,
                direction: position.direction,
            }
        } else {
            match wrap_around_method {
                WrapAroundMethod::In2d => self.wrap_around_in_2d(position),
                WrapAroundMethod::In3dCube => self.wrap_around_in_3d_cube(position),
            }
        }
    }

    fn wrap_around_in_2d(&self, position: &MapPosition) -> MapPosition {
        let mut current_shifted_square_coordinates =
            position.square_coordinates.shift_wrapping_around(
                position.direction,
                self.squares_width(),
                self.squares_height(),
            );

        while !self.is_square_present(current_shifted_square_coordinates) {
            current_shifted_square_coordinates = current_shifted_square_coordinates
                .shift_wrapping_around(
                    position.direction,
                    self.squares_width(),
                    self.squares_height(),
                );
        }

        let tile_coordinates = match position.direction {
            Up => Coordinates {
                x: position.tile_coordinates.x,
                y: self.square_size - 1,
            },
            Right => Coordinates {
                x: 0,
                y: position.tile_coordinates.y,
            },
            Down => Coordinates {
                x: position.tile_coordinates.x,
                y: 0,
            },
            Left => Coordinates {
                x: self.square_size - 1,
                y: position.tile_coordinates.y,
            },
        };

        MapPosition {
            square_coordinates: current_shifted_square_coordinates,
            tile_coordinates,
            direction: position.direction,
        }
    }

    fn wrap_around_in_3d_cube(&self, position: &MapPosition) -> MapPosition {
        let square_edge = SquareEdge {
            coordinates: position.square_coordinates,
            direction: position.direction,
        };
        let index_on_adjacent_edge = self.square_size
            - match position.direction {
                Up => position.tile_coordinates.x,
                Right => position.tile_coordinates.y,
                Down => self.square_size - position.tile_coordinates.x - 1,
                Left => self.square_size - position.tile_coordinates.y - 1,
            }
            - 1;
        let adjacent_square_edge = self.adjacent_edges.get(&square_edge).unwrap();
        let tile_coordinates = match adjacent_square_edge.direction {
            Up => Coordinates {
                x: index_on_adjacent_edge,
                y: 0,
            },
            Right => Coordinates {
                x: self.square_size - 1,
                y: index_on_adjacent_edge,
            },
            Down => Coordinates {
                x: self.square_size - index_on_adjacent_edge - 1,
                y: self.square_size - 1,
            },
            Left => Coordinates {
                x: 0,
                y: self.square_size - index_on_adjacent_edge - 1,
            },
        };
        MapPosition {
            square_coordinates: adjacent_square_edge.coordinates,
            tile_coordinates,
            direction: adjacent_square_edge.direction.opposite(),
        }
    }

    fn squares_width(&self) -> usize {
        self.squares[0].len()
    }

    fn squares_height(&self) -> usize {
        self.squares.len()
    }

    fn is_square_present(&self, coordinates: Coordinates) -> bool {
        self.squares[coordinates.y][coordinates.x].is_some()
    }
}

#[derive(Debug, Clone)]
struct Square {
    tile_rows: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Go(usize),
    TurnClockwise,
    TurnCounterClockwise,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn shift(self, direction: Direction, width: usize, height: usize) -> Option<Coordinates> {
        match direction {
            Up => {
                if self.y > 0 {
                    Some(Coordinates {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Right => {
                if self.x < width - 1 {
                    Some(Coordinates {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Down => {
                if self.y < height - 1 {
                    Some(Coordinates {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Left => {
                if self.x > 0 {
                    Some(Coordinates {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn shift_wrapping_around(
        self,
        direction: Direction,
        width: usize,
        height: usize,
    ) -> Coordinates {
        if let Some(shifted_normally) = self.shift(direction, width, height) {
            shifted_normally
        } else {
            match direction {
                Up => Coordinates {
                    x: self.x,
                    y: height - 1,
                },
                Right => Coordinates { x: 0, y: self.y },
                Down => Coordinates { x: self.x, y: 0 },
                Left => Coordinates {
                    x: width - 1,
                    y: self.y,
                },
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}

lazy_static! {
    static ref DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];
}

fn directions_starting_from(d: Direction) -> [Direction; 4] {
    let mut result = DIRECTIONS.clone();
    result.rotate_left(
        DIRECTIONS
            .iter()
            .enumerate()
            .find(|(_, od)| **od == d)
            .map(|(i, _)| i)
            .unwrap(),
    );
    result
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct SquareEdge {
    coordinates: Coordinates,
    direction: Direction,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum DieSide {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct DieEdge {
    side: DieSide,
    direction: Direction,
}

lazy_static! {
    static ref DIE_EDGES: HashMap<DieEdge, DieEdge> = [
        (
            DieEdge {
                side: One,
                direction: Up
            },
            DieEdge {
                side: Five,
                direction: Up
            }
        ),
        (
            DieEdge {
                side: One,
                direction: Right
            },
            DieEdge {
                side: Four,
                direction: Right
            }
        ),
        (
            DieEdge {
                side: One,
                direction: Down
            },
            DieEdge {
                side: Two,
                direction: Left
            }
        ),
        (
            DieEdge {
                side: One,
                direction: Left
            },
            DieEdge {
                side: Three,
                direction: Down
            }
        ),
        (
            DieEdge {
                side: Two,
                direction: Up
            },
            DieEdge {
                side: Four,
                direction: Up
            }
        ),
        (
            DieEdge {
                side: Two,
                direction: Right
            },
            DieEdge {
                side: Six,
                direction: Right
            }
        ),
        (
            DieEdge {
                side: Two,
                direction: Down
            },
            DieEdge {
                side: Three,
                direction: Left
            }
        ),
        (
            DieEdge {
                side: Three,
                direction: Up
            },
            DieEdge {
                side: Six,
                direction: Up
            }
        ),
        (
            DieEdge {
                side: Three,
                direction: Right
            },
            DieEdge {
                side: Five,
                direction: Right
            }
        ),
        (
            DieEdge {
                side: Four,
                direction: Down
            },
            DieEdge {
                side: Five,
                direction: Left
            }
        ),
        (
            DieEdge {
                side: Four,
                direction: Left
            },
            DieEdge {
                side: Six,
                direction: Down
            }
        ),
        (
            DieEdge {
                side: Five,
                direction: Down
            },
            DieEdge {
                side: Six,
                direction: Left
            }
        ),
    ]
    .into_iter()
    .flat_map(|(e1, e2)| vec![(e1.clone(), e2.clone()), (e2, e1)])
    .collect();
}

#[derive(Clone, Copy)]
struct MapPosition {
    square_coordinates: Coordinates,
    tile_coordinates: Coordinates,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum WrapAroundMethod {
    In2d,
    In3dCube,
}

#[cfg(test)]
mod tests {
    use crate::{parse_map, WrapAroundMethod};

    #[test]
    fn example_from_description() {
        let lines = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        let map = parse_map(lines).unwrap();

        assert_eq!(6032, map.final_password(WrapAroundMethod::In2d));
        assert_eq!(5031, map.final_password(WrapAroundMethod::In3dCube));
    }
}
