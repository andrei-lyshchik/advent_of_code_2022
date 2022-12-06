use std::ops::RangeInclusive;

use aoc2022::input::read_lines;

fn main() {
    let range_pairs: Vec<RangePair> = read_lines("inputs/day4.txt")
        .map(|l| RangePair::parse(&l).unwrap())
        .collect();
    let one_includes_other_count = range_pairs
        .iter()
        .filter(|rp| rp.one_includes_other())
        .count();

    println!(
        "Part 1: one includes other count: {}",
        one_includes_other_count
    );

    let one_overlaps_with_other_count = range_pairs.iter().filter(|rp| rp.one_overlaps_with_other()).count();

    println!(
        "Part 2: one overlaps with other count: {}",
        one_overlaps_with_other_count
    );
}

struct RangePair {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl RangePair {
    fn parse(line: &str) -> Option<RangePair> {
        let splitted: Vec<&str> = line.split(",").collect();
        if splitted.len() != 2 {
            return None;
        }

        match (
            Self::parse_range(splitted[0]),
            Self::parse_range(splitted[1]),
        ) {
            (Some(first), Some(second)) => Some(RangePair { first, second }),
            _ => None,
        }
    }

    fn parse_range(line_part: &str) -> Option<RangeInclusive<usize>> {
        let splitted: Vec<&str> = line_part.split("-").collect();
        if splitted.len() != 2 {
            return None;
        }

        match (
            splitted[0].parse::<usize>().ok(),
            splitted[1].parse::<usize>().ok(),
        ) {
            (Some(start), Some(end)) => Some(start..=end),
            _ => None,
        }
    }

    fn one_includes_other(self: &Self) -> bool {
        self.first.contains(self.second.start()) && self.first.contains(self.second.end())
            || self.second.contains(self.first.start()) && self.second.contains(self.first.end())
    }

    fn one_overlaps_with_other(self: &Self) -> bool {
        !(self.first.end() < self.second.start() || self.second.end() < self.first.start())
    }
}
