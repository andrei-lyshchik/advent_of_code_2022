#[macro_use]
extern crate lazy_static;

use aoc2022::input::read_lines;
use regex::Regex;

fn main() {
    let mut crates_lines: Vec<Vec<char>> = vec![];
    let mut commands: Vec<Command> = vec![];
    for line in read_lines("inputs/day5.txt") {
        if line.starts_with("[") {
            crates_lines.push(line.chars().collect());
        } else if line.starts_with("move") {
            commands.push(Command::parse(&line).unwrap());
        }
    }
    let mut crates = State::parse(crates_lines).unwrap();
    let mut crates_copy = crates.clone();
    crates.apply(&commands, false);
    println!(
        "Part 1: crates on top: {}",
        crates.crates_on_top_of_stacks()
    );

    crates_copy.apply(&commands, true);
    println!(
        "Part 2: crates on top while retaining order during moves: {}",
        crates_copy.crates_on_top_of_stacks()
    )
}

#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

impl Command {
    fn parse(line: &str) -> Option<Command> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();
        }

        let captures: Vec<_> = RE.captures_iter(line).collect();
        if captures.len() != 1 {
            return None;
        }

        if captures[0].len() != 4 {
            return None;
        }
        Some(Command {
            count: captures[0][1].parse::<usize>().unwrap(),
            from: captures[0][2].parse::<usize>().unwrap(),
            to: captures[0][3].parse::<usize>().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn parse(crates_rows: Vec<Vec<char>>) -> Option<State> {
        if crates_rows.len() == 0 {
            return None;
        }
        let crates_count = (crates_rows[0].len() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); crates_count];

        for crates_row in crates_rows {
            for i in 0..crates_count {
                if crates_row[i * 4 + 1] != ' ' {
                    stacks[i].push(crates_row[i * 4 + 1]);
                }
            }
        }
        for i in 0..crates_count {
            stacks[i].reverse()
        }

        Some(State { stacks })
    }

    fn apply(self: &mut Self, commands: &Vec<Command>, retain_order: bool) {
        for command in commands {
            let len_before = self.stacks[command.from - 1].len();
            let mut to_move: Vec<char> = if retain_order {
                self.stacks[command.from - 1][len_before - command.count..len_before]
                    .iter()
                    .cloned()
                    .collect()
            } else {
                self.stacks[command.from - 1]
                    .iter()
                    .rev()
                    .take(command.count)
                    .cloned()
                    .collect()
            };
            self.stacks[command.to - 1].append(&mut to_move);
            self.stacks[command.from - 1].truncate(len_before - command.count);
        }
    }

    fn crates_on_top_of_stacks(self: &Self) -> String {
        self.stacks.iter().flat_map(|s| s.last()).collect()
    }
}
