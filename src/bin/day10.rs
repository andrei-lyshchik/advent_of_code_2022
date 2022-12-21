use aoc2022::input::read_lines;

fn main() {
    let instructions: Vec<_> = read_lines("inputs/day10.txt")
        .map(|l| Instruction::parse(&l).unwrap())
        .collect();

    let signal_strength_sum: i32 = observe_signal_strength_value(&instructions, 20, 40)
        .iter()
        .take(6)
        .sum();
    println!("Part 1: signal strength sum: {}", signal_strength_sum);

    println!("Part 2:");
    let rendered = draw(&instructions);
    for row in rendered {
        println!("{}", row.iter().collect::<String>());
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction, String> {
        if line.starts_with("noop") {
            Ok(Instruction::Noop)
        } else if line.starts_with("addx ") {
            let value = match line[5..].parse() {
                Ok(value) => value,
                Err(err) => return Err(format!("Can't parse addx value: {}", err)),
            };
            Ok(Instruction::Addx(value))
        } else {
            Err(format!("Can't parse instruction out of '{}'", line))
        }
    }
}

fn observe_signal_strength_value(
    instructions: &Vec<Instruction>,
    first_cycle_to_observe: usize,
    step: usize,
) -> Vec<i32> {
    let mut result = vec![];
    let mut next_cycle_to_observe = first_cycle_to_observe;
    let mut current_cycle = 1;
    let mut current_value = 1;
    for instruction in instructions {
        for _ in 0..cycles(instruction) {
            if current_cycle == next_cycle_to_observe {
                result.push(current_value * (current_cycle as i32));
                next_cycle_to_observe += step;
            }
            current_cycle += 1;
        }
        if let Instruction::Addx(add_value) = instruction {
            current_value += add_value;
        }
    }
    if current_cycle == next_cycle_to_observe {
        result.push(current_value * (current_cycle as i32));
    }
    result
}

fn cycles(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::Noop => 1,
        Instruction::Addx(_) => 2,
    }
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn draw(instructions: &Vec<Instruction>) -> [[char; WIDTH]; HEIGHT] {
    let mut result = [['.'; WIDTH]; HEIGHT];
    let mut sprite_center: i32 = 1;
    let mut drawing_row = 0;
    let mut drawing_col: usize = 0;
    for instruction in instructions {
        for _ in 0..cycles(instruction) {
            if drawing_col as i32 >= sprite_center - 1 && drawing_col as i32 <= sprite_center + 1 {
                result[drawing_row][drawing_col] = '#';
            }
            if drawing_col == WIDTH - 1 {
                drawing_col = 0;
                drawing_row = (drawing_row + 1) % HEIGHT;
            } else {
                drawing_col += 1;
            }
        }
        if let Instruction::Addx(add_value) = instruction {
            sprite_center += add_value;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{observe_signal_strength_value, Instruction};

    #[test]
    fn test_simple_example() {
        let result = observe_signal_strength_value(
            &vec![
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5),
            ],
            1,
            1,
        );

        assert_eq!(vec![1 * 1, 1 * 2, 1 * 3, 4 * 4, 4 * 5, -1 * 6], result);
    }
}
