use std::collections::HashSet;

use aoc2022::input::read_lines;

fn main() {
    let line: Vec<char> = read_lines("inputs/day6.txt")
        .next()
        .unwrap()
        .chars()
        .collect();

    println!(
        "Part 1: first marker after character {:?}",
        first_unique_sequence_end_position(&line, 4)
    );
    println!(
        "Part 2: first marker after character {:?}",
        first_unique_sequence_end_position(&line, 14)
    );
}

fn first_unique_sequence_end_position(line: &Vec<char>, sequence_length: usize) -> Option<usize> {
    for i in (sequence_length - 1)..line.len() - 1 {
        let sequence = line[i - (sequence_length - 1)..=i]
            .iter()
            .collect::<HashSet<_>>();
        if sequence.len() == sequence_length {
            return Some(i + 1);
        }
    }
    return None;
}
