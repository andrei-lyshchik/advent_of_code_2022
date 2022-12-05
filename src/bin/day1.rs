use aoc2022::input::read_lines;

fn main() {
    let mut elfs_calories: Vec<Vec<i32>> = Vec::new();
    let mut current_elf: Vec<i32> = Vec::new();
    for line in read_lines("inputs/day1.txt") {
        if !line.is_empty() {
            current_elf.push(line.parse().unwrap());
        } else {
            elfs_calories.push(current_elf.clone());
            current_elf.clear();
        }
    }

    let mut elfs_calories_sums: Vec<i32> = elfs_calories
        .iter()
        .map(|calories| calories.iter().sum::<i32>())
        .collect();
    elfs_calories_sums.sort();

    println!(
        "Part 1: max calories: {}",
        elfs_calories_sums.last().unwrap()
    );
    println!(
        "Part 2: sum of 3 top calories: {}",
        elfs_calories_sums[elfs_calories_sums.len() - 3]
            + elfs_calories_sums[elfs_calories_sums.len() - 2]
            + elfs_calories_sums[elfs_calories_sums.len() - 1]
    )
}
