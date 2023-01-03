use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
};

use aoc2022::input::read_lines;

#[macro_use]
extern crate lazy_static;

fn main() {
    let pushes = match read_lines("inputs/day17.txt")
        .next()
        .unwrap()
        .chars()
        .map(|c| parse_push(c))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(pushes) => pushes,
        Err(err) => {
            println!("Can't parse pushes: {}", err);
            return;
        }
    };
    println!("Part 1: {}", calculate_height_after_rocks(2022, &pushes));
    println!(
        "Part 2: {}",
        calculate_height_after_rocks(1_000_000_000_000, &pushes)
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Push {
    Left,
    Right,
}

fn parse_push(char: char) -> Result<Push, String> {
    match char {
        '<' => Ok(Push::Left),
        '>' => Ok(Push::Right),
        unexpected @ _ => Err(format!("Can't parse push out of '{}'", unexpected)),
    }
}

struct PushesEndlessIterator<'a> {
    pushes: &'a [Push],
    i: usize,
}

impl<'a> PushesEndlessIterator<'a> {
    fn new(pushes: &'a [Push]) -> PushesEndlessIterator<'a> {
        PushesEndlessIterator { pushes, i: 0 }
    }

    fn next(&mut self) -> Push {
        let element = self.pushes[self.i];
        self.i = (self.i + 1) % self.pushes.len();
        element
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum RockShape {
    HorizontalBar,
    Cross,
    HockeyStick,
    VerticalBar,
    Square,
}

impl RockShape {
    fn deltas(self) -> &'static Vec<Point> {
        lazy_static! {
            static ref HORIZONTAL_BAR_DELTAS: Vec<Point> = vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ];
            static ref CROSS_DELTAS: Vec<Point> = vec![
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(2, -1),
                Point::new(1, -2),
            ];
            static ref HOCKEY_STICK_DELTAS: Vec<Point> = vec![
                Point::new(2, 0),
                Point::new(2, -1),
                Point::new(2, -2),
                Point::new(1, -2),
                Point::new(0, -2),
            ];
            static ref VERTICAL_BAR_DELTAS: Vec<Point> = vec![
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2),
                Point::new(0, -3),
            ];
            static ref SQUARE_DELTAS: Vec<Point> = vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(1, -1),
            ];
        }
        match self {
            RockShape::HorizontalBar => &*HORIZONTAL_BAR_DELTAS,
            RockShape::Cross => &*CROSS_DELTAS,
            RockShape::HockeyStick => &*HOCKEY_STICK_DELTAS,
            RockShape::VerticalBar => &*VERTICAL_BAR_DELTAS,
            RockShape::Square => &*SQUARE_DELTAS,
        }
    }

    fn height(&self) -> i64 {
        match self {
            RockShape::HorizontalBar => 1,
            RockShape::Cross => 3,
            RockShape::HockeyStick => 3,
            RockShape::VerticalBar => 4,
            RockShape::Square => 2,
        }
    }
}

struct Rock {
    rock_shape: RockShape,
    top_left: Point,
    points: Vec<Point>,
}

impl Rock {
    fn new(rock_shape: RockShape, top_left: Point) -> Rock {
        Rock {
            rock_shape,
            top_left,
            points: rock_shape.deltas().iter().map(|d| &top_left + d).collect(),
        }
    }

    fn push(&self, push: Push) -> Rock {
        let new_top_left = match push {
            Push::Left => Point::new(self.top_left.x - 1, self.top_left.y),
            Push::Right => Point::new(self.top_left.x + 1, self.top_left.y),
        };
        Rock::new(self.rock_shape, new_top_left)
    }

    fn push_down(&self) -> Rock {
        let new_top_left = Point::new(self.top_left.x, self.top_left.y - 1);
        Rock::new(self.rock_shape, new_top_left)
    }
}

struct RockShapesEndlessIterator {
    i: usize,
}

impl RockShapesEndlessIterator {
    fn new() -> RockShapesEndlessIterator {
        RockShapesEndlessIterator { i: 0 }
    }

    fn next(&mut self) -> RockShape {
        let result = match self.i {
            0 => RockShape::HorizontalBar,
            1 => RockShape::Cross,
            2 => RockShape::HockeyStick,
            3 => RockShape::VerticalBar,
            4 => RockShape::Square,
            _ => panic!("Illegal constructed value of i: {}", self.i),
        };
        self.i = (self.i + 1) % 5;
        result
    }
}

impl Iterator for RockShapesEndlessIterator {
    type Item = RockShape;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.i {
            0 => RockShape::HorizontalBar,
            1 => RockShape::Cross,
            2 => RockShape::HockeyStick,
            3 => RockShape::VerticalBar,
            4 => RockShape::Square,
            _ => panic!("Illegal constructed value of i: {}", self.i),
        };
        self.i = (self.i + 1) % 5;
        Some(result)
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const WIDTH: usize = 7;

#[derive(PartialEq, Eq, Hash)]
struct State {
    normalized_top_edge: TopEdge,
    shape_i: usize,
    push_i: usize,
}

struct Observation {
    after_rocks: usize,
    height: usize,
}

fn calculate_height_after_rocks(rocks_count: usize, pushes: &[Push]) -> usize {
    let mut top_edge = TopEdge::new();
    let mut shapes = RockShapesEndlessIterator::new();
    let mut pushes = PushesEndlessIterator::new(&pushes);
    calculate_height_after_rocks_helper(rocks_count, &mut top_edge, &mut shapes, &mut pushes)
}

fn calculate_height_after_rocks_helper(
    rocks_count: usize,
    top_edge: &mut TopEdge,
    shapes: &mut RockShapesEndlessIterator,
    pushes: &mut PushesEndlessIterator,
) -> usize {
    let mut observed_states = HashMap::new();
    observed_states.insert(
        State {
            normalized_top_edge: top_edge.normalized(),
            shape_i: shapes.i,
            push_i: pushes.i,
        },
        Observation {
            after_rocks: 0,
            height: top_edge.height(),
        },
    );
    for rocks in 1..=rocks_count {
        top_edge.simulate_fall(shapes.next(), pushes);
        let state = State {
            normalized_top_edge: top_edge.normalized(),
            shape_i: shapes.i,
            push_i: pushes.i,
        };
        if let Some(already_observed) = observed_states.get(&state) {
            let cycle_length = rocks - already_observed.after_rocks;
            let cycle_y_diff = top_edge.height() - already_observed.height;
            let rocks_left = rocks_count - rocks;
            return rocks_left / cycle_length * cycle_y_diff
                + calculate_height_after_rocks_helper(
                    rocks_left % cycle_length,
                    top_edge,
                    shapes,
                    pushes,
                );
        } else {
            observed_states.insert(
                state,
                Observation {
                    after_rocks: rocks,
                    height: top_edge.height(),
                },
            );
        }
    }
    top_edge.height()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TopEdge {
    base_heights: [usize; WIDTH],
    holes_above_base: Vec<VecDeque<bool>>,
}

impl TopEdge {
    fn new() -> TopEdge {
        TopEdge {
            base_heights: [0; WIDTH],
            holes_above_base: vec![VecDeque::new(); WIDTH],
        }
    }

    fn simulate_fall(&mut self, shape: RockShape, pushes: &mut PushesEndlessIterator) {
        let start = self.start(shape);
        let mut current_rock = Rock::new(shape, start);
        loop {
            let pushed = current_rock.push(pushes.next());
            if self.valid_placement(&pushed) {
                current_rock = pushed;
            }
            let pushed_down = current_rock.push_down();
            if !self.valid_placement(&pushed_down) {
                break;
            } else {
                current_rock = pushed_down;
            }
        }
        self.place(&current_rock);
    }

    fn start(&self, shape: RockShape) -> Point {
        Point::new(2, (self.height() + 2 + shape.height() as usize) as i64)
    }

    fn height(&self) -> usize {
        (0..WIDTH)
            .map(|x| self.base_heights[x] + self.holes_above_base[x].len())
            .max()
            .unwrap()
    }

    fn valid_placement(&self, rock: &Rock) -> bool {
        rock.points.iter().all(|c| self.valid_point(c))
    }

    fn valid_point(&self, point: &Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        let x = point.x as usize;
        let y = point.y as usize;
        if x >= WIDTH || y < self.base_heights[x] {
            return false;
        };
        let y_in_holes = y - self.base_heights[x];
        y_in_holes >= self.holes_above_base[x].len() || self.holes_above_base[x][y_in_holes]
    }

    fn place(&mut self, rock: &Rock) {
        for point in rock.points.iter() {
            let x = point.x as usize;
            let y_in_terrain = point.y as usize - self.base_heights[x];

            while self.holes_above_base[x].len() <= y_in_terrain {
                self.holes_above_base[x].push_back(true);
            }
            self.holes_above_base[x][y_in_terrain] = false;
        }

        let desired_height = (0..WIDTH)
            .map(|x| self.base_heights[x] + self.holes_above_base[x].len())
            .max()
            .unwrap()
            + 1;

        for x in 0..WIDTH {
            while self.base_heights[x] + self.holes_above_base[x].len() < desired_height {
                self.holes_above_base[x].push_back(true);
            }
        }

        let (base_height_diffs, new_holes_above_base) =
            self.calculate_base_heights_diffs_and_new_holes(desired_height);
        for x in 0..WIDTH {
            self.base_heights[x] += base_height_diffs[x];
        }
        self.holes_above_base = new_holes_above_base;
    }

    fn calculate_base_heights_diffs_and_new_holes(
        &self,
        height: usize,
    ) -> ([usize; WIDTH], Vec<VecDeque<bool>>) {
        let mut reachable_holes = vec![];
        for x in 0..WIDTH {
            reachable_holes.push(VecDeque::from(vec![false; self.holes_above_base[x].len()]));
        }
        let mut queue = VecDeque::new();
        queue.push_back((0 as usize, height - 1));
        while let Some((x, y)) = queue.pop_front() {
            if reachable_holes[x][y - self.base_heights[x]] {
                continue;
            }
            reachable_holes[x][y - self.base_heights[x]] = true;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if dx == -1 && x == 0 || dx == 1 && x == WIDTH - 1 {
                        continue;
                    }
                    if dy == -1 && y == 0 {
                        continue;
                    }
                    let x_neighbor = (x as i32 + dx) as usize;
                    let y_neighbor = (y as i32 + dy) as usize;

                    if y_neighbor < self.base_heights[x_neighbor]
                        || (y_neighbor - self.base_heights[x_neighbor]
                            >= self.holes_above_base[x_neighbor].len())
                    {
                        continue;
                    }
                    if !self.holes_above_base[x_neighbor]
                        [y_neighbor - self.base_heights[x_neighbor]]
                    {
                        continue;
                    }
                    queue.push_back((x_neighbor, y_neighbor));
                }
            }
        }
        let mut base_height_increases = [0; WIDTH];
        for x in 0..WIDTH {
            while let Some(reachable_at_bottom) = reachable_holes[x].pop_front() {
                if !reachable_at_bottom {
                    base_height_increases[x] += 1;
                } else {
                    reachable_holes[x].push_front(reachable_at_bottom);
                    break;
                }
            }
            while let Some(reachable_at_top) = reachable_holes[x].pop_back() {
                if !reachable_at_top {
                    reachable_holes[x].push_back(reachable_at_top);
                    break;
                }
            }
        }
        (base_height_increases, reachable_holes)
    }

    fn normalized(&self) -> TopEdge {
        let mut normalized_base_heights = self.base_heights.clone();
        let min_height = self.base_heights.iter().min().unwrap();
        for x in 0..WIDTH {
            normalized_base_heights[x] -= min_height;
        }
        TopEdge {
            base_heights: normalized_base_heights,
            holes_above_base: self.holes_above_base.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_height_after_rocks, parse_push, Point, Rock, RockShape, TopEdge};
    use std::collections::VecDeque;

    #[test]
    fn example_from_description() {
        let pushes = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
            .chars()
            .map(|c| parse_push(c))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(3068, calculate_height_after_rocks(2022, &pushes));
    }

    #[test]
    fn test_placement_to_top_edge() {
        let mut top_edge = TopEdge::new();
        top_edge.place(&Rock::new(RockShape::Cross, Point::new(1, 2)));

        assert_eq!(
            TopEdge {
                base_heights: [0, 0, 3, 0, 0, 0, 0],
                holes_above_base: vec![
                    VecDeque::new(),
                    VecDeque::from([true, false]),
                    VecDeque::new(),
                    VecDeque::from([true, false]),
                    VecDeque::new(),
                    VecDeque::new(),
                    VecDeque::new(),
                ]
            },
            top_edge,
        );

        top_edge.place(&Rock::new(RockShape::HockeyStick, Point::new(3, 2)));
        assert_eq!(
            TopEdge {
                base_heights: [0, 0, 3, 2, 1, 3, 0],
                holes_above_base: vec![
                    VecDeque::new(),
                    VecDeque::from([true, false]),
                    VecDeque::new(),
                    VecDeque::new(),
                    VecDeque::new(),
                    VecDeque::new(),
                    VecDeque::new(),
                ]
            },
            top_edge,
        )
    }
}
