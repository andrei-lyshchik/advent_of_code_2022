extern crate bit_set;

use bit_set::BitSet;

use aoc2022::input::read_lines;

fn main() {
    let priorities_sum: usize = read_lines("inputs/day3.txt")
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|l| calculate_common_item_type_priority(&l))
        .sum();

    println!("Part 1: priorities sum: {}", priorities_sum);

    let item_types = read_lines("inputs/day3.txt")
        .map(|l| to_priorities_set(&l))
        .collect::<Vec<BitSet<u32>>>();

    println!(
        "Part 2: group common item type priorities sum: {:?}",
        groups_common_item_type_priorities(&item_types)
    )
}

fn priority(item_type: char) -> usize {
    match item_type {
        'a'..='z' => (item_type as usize) - ('a' as usize) + 1,
        'A'..='Z' => (item_type as usize) - ('A' as usize) + 27,
        _ => 0,
    }
}

fn calculate_common_item_type_priority(item_types: &Vec<char>) -> usize {
    let first_compartment = item_types[0..item_types.len() / 2]
        .iter()
        .map(|c| priority(*c))
        .collect::<BitSet>();
    let second_compartment = item_types[item_types.len() / 2..item_types.len()]
        .iter()
        .map(|c| priority(*c))
        .collect::<BitSet>();

    first_compartment.intersection(&second_compartment).sum()
}

fn to_priorities_set(item_types: &str) -> BitSet<u32> {
    let mut res: BitSet<u32> = BitSet::with_capacity(52);
    for c in item_types.chars() {
        res.insert((priority(c)) as usize);
    }
    res
}

fn group_common_item_type_priority(
    item_types_1: &BitSet<u32>,
    item_types_2: &BitSet<u32>,
    item_types_3: &BitSet<u32>,
) -> Option<usize> {
    let first_two_intersection = item_types_1
        .intersection(item_types_2)
        .collect::<BitSet<u32>>();
    let all_intersection = first_two_intersection
        .intersection(item_types_3)
        .collect::<Vec<usize>>();
    if all_intersection.len() == 1 {
        all_intersection.first().cloned()
    } else {
        None
    }
}

fn groups_common_item_type_priorities(item_types: &Vec<BitSet<u32>>) -> Option<usize> {
    let mut priorities_sum: usize = 0;
    let mut i = 0;
    while i < item_types.len() - 2 {
        if let Some(group_priority) =
            group_common_item_type_priority(&item_types[i], &item_types[i + 1], &item_types[i + 2])
        {
            priorities_sum += group_priority;
        } else {
            return None;
        }
        i += 3;
    }
    Some(priorities_sum)
}
