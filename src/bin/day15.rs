use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

use aoc2022::input::read_lines;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn main() {
    let observations: Vec<Observation> =
        match read_lines("inputs/day15.txt").map(|l| l.parse()).collect() {
            Ok(observations) => observations,
            Err(err) => {
                println!("Couldn't parse observations: {}", err);
                return;
            }
        };
    println!(
        "Part 1: {:?}",
        positions_not_containing_beacons_at_y(&observations, 2000000)
    );
    let coordinates_possibly_containing_beacon =
        coordinates_possibly_containing_beacon(&observations, 0..=4000000, 0..=4000000);
    if coordinates_possibly_containing_beacon.len() == 1 {
        println!(
            "Part 2: {}",
            coordinates_possibly_containing_beacon[0].tuning_frequency()
        );
    } else {
        println!(
            "Unable to find only a single coordinate possibly containing beacon, found: {:?}",
            coordinates_possibly_containing_beacon
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance(self: &Self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn tuning_frequency(self: &Self) -> i64 {
        (self.x as i64) * 4000000 + (self.y as i64)
    }
}

#[derive(Debug)]
struct Observation {
    sensor_at: Coordinate,
    closest_beacon_at: Coordinate,
}

impl FromStr for Observation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }
        RE.captures(s)
            .and_then(|captures| -> Option<Observation> {
                match (
                    captures.get(1).unwrap().as_str().parse(),
                    captures.get(2).unwrap().as_str().parse(),
                    captures.get(3).unwrap().as_str().parse(),
                    captures.get(4).unwrap().as_str().parse(),
                ) {
                    (
                        Ok(sensor_at_x),
                        Ok(sensor_at_y),
                        Ok(closest_beacon_at_x),
                        Ok(closest_beacon_at_y),
                    ) => Some(Observation {
                        sensor_at: Coordinate {
                            x: sensor_at_x,
                            y: sensor_at_y,
                        },
                        closest_beacon_at: Coordinate {
                            x: closest_beacon_at_x,
                            y: closest_beacon_at_y,
                        },
                    }),
                    _ => None,
                }
            })
            .ok_or(format!("Couldn't parse observation out of '{}'", s))
    }
}

impl Observation {
    fn observed_x_range_at_y(self: &Self, y: i32) -> Option<RangeInclusive<i32>> {
        let distance_to_beacon = self.sensor_at.manhattan_distance(&self.closest_beacon_at);
        let y_diff_from_sensor = (self.sensor_at.y - y).abs();
        if y_diff_from_sensor > distance_to_beacon {
            return None;
        }
        Some(
            self.sensor_at.x - distance_to_beacon + y_diff_from_sensor
                ..=self.sensor_at.x + distance_to_beacon - y_diff_from_sensor,
        )
    }
}

fn to_disjoint_ranges(mut ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut result = vec![];
    let mut current_start = *ranges[0].start();
    let mut current_end = *ranges[0].end();
    for range in ranges[1..].iter() {
        if *range.start() <= current_end + 1 {
            if current_end < *range.end() {
                current_end = *range.end();
            }
        } else {
            result.push(current_start..=current_end);
            current_start = *range.start();
            current_end = *range.end();
        }
    }
    result.push(current_start..=current_end);
    result
}

fn disjoint_observed_x_ranges_at_y(
    observations: &[Observation],
    y: i32,
) -> Vec<RangeInclusive<i32>> {
    let observed_x_ranges_at_y: Vec<_> = observations
        .iter()
        .filter_map(|o| o.observed_x_range_at_y(y))
        .collect();

    to_disjoint_ranges(observed_x_ranges_at_y)
}

fn positions_not_containing_beacons_at_y(observations: &[Observation], y: i32) -> i32 {
    let disjoint_observed_x_ranges_at_y = disjoint_observed_x_ranges_at_y(observations, y);

    let mut known_beacons_iter = {
        let mut known_beacons_xs = observations
            .iter()
            .filter_map(|o| {
                if o.closest_beacon_at.y == y {
                    Some(o.closest_beacon_at.x)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        known_beacons_xs.sort();
        known_beacons_xs.into_iter().peekable()
    };

    let mut result = 0;
    for range in disjoint_observed_x_ranges_at_y {
        result += range.end() - range.start() + 1;
        while let Some(known_beacon_x) = known_beacons_iter.peek() {
            if !range.contains(known_beacon_x) {
                break;
            }
            result -= 1;
            known_beacons_iter.next();
        }
    }
    result
}

fn coordinates_possibly_containing_beacon(
    observations: &[Observation],
    x_range_to_try: RangeInclusive<i32>,
    y_range_to_try: RangeInclusive<i32>,
) -> Vec<Coordinate> {
    let mut result = vec![];
    for y in y_range_to_try {
        let observed_x_ranges_at_y = disjoint_observed_x_ranges_at_y(observations, y);
        let mut current_x = *x_range_to_try.start();
        for range in observed_x_ranges_at_y.iter() {
            if current_x > *x_range_to_try.end() {
                break;
            }
            for x in current_x..*range.start() {
                result.push(Coordinate { x, y });
            }
            current_x = range.end() + 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates_possibly_containing_beacon, positions_not_containing_beacons_at_y,
        to_disjoint_ranges, Coordinate, Observation,
    };

    #[test]
    fn test_observed_range_at_y() {
        let observation = Observation {
            sensor_at: Coordinate { x: 8, y: 7 },
            closest_beacon_at: Coordinate { x: 2, y: 10 },
        };
        assert_eq!(Some(1..=15), observation.observed_x_range_at_y(9));
        assert_eq!(Some(5..=11), observation.observed_x_range_at_y(1));
        assert_eq!(Some(8..=8), observation.observed_x_range_at_y(-2));
        assert_eq!(None, observation.observed_x_range_at_y(-3));
    }

    #[test]
    fn test_to_disjoint_ranges() {
        assert_eq!(
            vec![1..=10, 12..=20],
            to_disjoint_ranges(vec![1..=5, 6..=7, 7..=10, 7..=9, 12..=16, 13..=20],)
        )
    }

    #[test]
    fn example_from_description_part1() {
        let observations: Vec<Observation> = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Observation>, _>>()
            .unwrap();

        assert_eq!(26, positions_not_containing_beacons_at_y(&observations, 10));
    }

    #[test]
    fn example_from_description_part2() {
        let observations: Vec<Observation> = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Observation>, _>>()
            .unwrap();

        assert_eq!(
            vec![Coordinate { x: 14, y: 11 }],
            coordinates_possibly_containing_beacon(&observations, 0..=20, 0..=20)
        );
    }
}
