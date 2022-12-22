use std::str::FromStr;

use aoc2022::input::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/day11.txt").collect();
    if let Ok(monkeys) = parse_monkeys(lines) {
        if !co_prime(monkeys.iter().map(|m| m.throw_test.divisible_by).collect()) {
            println!("Can't use modulo calculation - divisible_by nums are not co-prime");
            return;
        }
        let counts_part_1 = calculate_inspected_item_counts(&monkeys, 20, 3);
        println!(
            "Part 1: monkey business level: {}",
            monkey_business_level(counts_part_1)
        );

        let counts_part_2 = calculate_inspected_item_counts(&monkeys, 10000, 1);
        println!(
            "Part 2: monkey business level: {}",
            monkey_business_level(counts_part_2),
        );
    }
}

fn co_prime(nums: Vec<u64>) -> bool {
    for i in 0..nums.len() - 1 {
        for j in (i + 1)..nums.len() {
            for factor in 2..nums[i].min(nums[j]) / 2 {
                if nums[i] % factor == 0 && nums[j] % factor == 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn calculate_inspected_item_counts(
    initial_state: &Vec<Monkey>,
    rounds: usize,
    divide_item_level_after_inspection_by: u64,
) -> Vec<usize> {
    let modulo = match initial_state
        .iter()
        .map(|m| m.throw_test.divisible_by)
        .reduce(|acc, d| acc * d)
    {
        Some(divisors_multiplication) => divisors_multiplication,
        None => return vec![],
    };
    let mut current_state = initial_state.clone();
    let mut result = vec![0; current_state.len()];
    for _ in 0..rounds {
        for i in 0..current_state.len() {
            result[i] += current_state[i].item_count();
            let item_throws =
                current_state[i].calculate_throws(divide_item_level_after_inspection_by, modulo);
            current_state[i].remove_items();
            for item_throw in item_throws {
                current_state[item_throw.to_monkey_idx].add_item(item_throw.item_level);
            }
        }
    }
    result
}

fn monkey_business_level(mut counts: Vec<usize>) -> usize {
    counts.sort();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation_expression: BinaryExpression,
    throw_test: ThrowTest,
}

fn parse_monkeys(lines: Vec<String>) -> Result<Vec<Monkey>, String> {
    let mut result = vec![];
    let mut monkey_lines: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            result.push(parse_monkey(&monkey_lines)?);
            monkey_lines.clear();
        } else {
            monkey_lines.push(line);
        }
    }
    result.push(parse_monkey(&monkey_lines)?);

    Ok(result)
}

fn parse_monkey(lines: &[String]) -> Result<Monkey, String> {
    if lines.len() < 6 {
        return Err(format!("Can't parse monkey out of {:?}", lines));
    }
    Ok(Monkey {
        items: parse_items(&lines[1])?,
        operation_expression: parse_operation_expression(&lines[2])?,
        throw_test: parse_throw_test(&lines[3..6])?,
    })
}

fn parse_items(line: &str) -> Result<Vec<u64>, String> {
    const PREFIX: &str = "  Starting items: ";
    if line.starts_with(PREFIX) {
        line[PREFIX.len()..]
            .split(", ")
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Can't parse monkey items: {}", e))
    } else {
        Err(format!("Can't parse monkey items out of '{}'", line))
    }
}

fn parse_operation_expression(line: &str) -> Result<BinaryExpression, String> {
    const PREFIX: &str = "  Operation: new = ";
    if line.starts_with(PREFIX) {
        let parts: Vec<_> = line[PREFIX.len()..].split(" ").collect();
        let first_arg = parse_term(parts[0])?;
        let second_arg = parse_term(parts[2])?;
        let operation = parse_operation(&parts[1])?;
        Ok(BinaryExpression {
            first_arg,
            second_arg,
            operation,
        })
    } else {
        Err(format!("Can't parse binary expression out of '{}'", line))
    }
}

fn parse_term(str: &str) -> Result<Term, String> {
    if str == "old" {
        Ok(Term::OldValue)
    } else {
        let constant_value = str
            .parse::<u64>()
            .map_err(|e| format!("Can't parse expression term: {}", e))?;
        Ok(Term::Constant(constant_value))
    }
}

fn parse_operation(str: &str) -> Result<Operation, String> {
    match str {
        "*" => Ok(Operation::Multiplication),
        "+" => Ok(Operation::Plus),
        _ => Err(format!("Can't parse operation out of '{}'", str)),
    }
}

fn parse_throw_test(lines: &[String]) -> Result<ThrowTest, String> {
    if lines.len() < 3 {
        return Err(format!("Can't parse throw test out of {:?}", lines));
    }
    let divisible_by = parse_after_prefix(&lines[0], "  Test: divisible by ")?;
    let monkey_idx_if_true = parse_after_prefix(&lines[1], "    If true: throw to monkey ")?;
    let monkey_idx_if_false = parse_after_prefix(&lines[2], "    If false: throw to monkey ")?;
    Ok(ThrowTest {
        divisible_by,
        monkey_idx_if_true,
        monkey_idx_if_false,
    })
}

fn parse_after_prefix<T>(line: &str, expected_prefix: &str) -> Result<T, String>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    if line.starts_with(expected_prefix) {
        line[expected_prefix.len()..]
            .parse::<T>()
            .map_err(|e| format!("Can't parse number: {}", e))
    } else {
        Err(format!("Can't parse number, expected prefix: '{}'", line))
    }
}

struct ItemThrow {
    to_monkey_idx: usize,
    item_level: u64,
}

impl Monkey {
    fn calculate_throws(
        &self,
        divide_item_level_after_inspection_by: u64,
        modulo: u64,
    ) -> Vec<ItemThrow> {
        self.items
            .iter()
            .map(|il| {
                (self.operation_expression.calculate(*il) / divide_item_level_after_inspection_by)
                    % modulo
            })
            .map(|il| self.throw_test.calculate(il))
            .collect()
    }

    fn add_item(&mut self, item_level: u64) {
        self.items.push(item_level)
    }

    fn remove_items(&mut self) {
        self.items.clear()
    }

    fn item_count(self: &Self) -> usize {
        self.items.len()
    }
}

#[derive(Debug, Clone)]
struct BinaryExpression {
    first_arg: Term,
    second_arg: Term,
    operation: Operation,
}

impl BinaryExpression {
    fn calculate(self: &Self, old_value: u64) -> u64 {
        let first_arg_value = match self.first_arg {
            Term::Constant(value) => value,
            Term::OldValue => old_value,
        };
        let second_arg_value = match self.second_arg {
            Term::Constant(value) => value,
            Term::OldValue => old_value,
        };
        match self.operation {
            Operation::Plus => first_arg_value + second_arg_value,
            Operation::Multiplication => first_arg_value * second_arg_value,
        }
    }
}

#[derive(Debug, Clone)]
enum Term {
    Constant(u64),
    OldValue,
}

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Multiplication,
}

#[derive(Debug, Clone)]
struct ThrowTest {
    divisible_by: u64,
    monkey_idx_if_true: usize,
    monkey_idx_if_false: usize,
}

impl ThrowTest {
    fn calculate(self: &Self, item_level: u64) -> ItemThrow {
        let to_monkey_idx = if item_level % self.divisible_by == 0 {
            self.monkey_idx_if_true
        } else {
            self.monkey_idx_if_false
        };
        ItemThrow {
            to_monkey_idx,
            item_level: item_level.clone(),
        }
    }
}
