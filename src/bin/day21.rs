use std::collections::HashMap;

use aoc2022::input::read_lines;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn main() {
    let lines = read_lines("inputs/day21.txt").collect::<Vec<String>>();
    let yell_job_expressions =
        match parse_jobs(&lines).and_then(|jobs| YellJobExpressions::new(jobs)) {
            Ok(jobs) => jobs,
            Err(err) => {
                println!("Can't parse monkey jobs or expressions: {}", err);
                return;
            }
        };

    println!(
        "Part 1: {:?}",
        yell_job_expressions.root_expression_value_as_in_first_part()
    );
    println!(
        "Part 2: {:?}",
        yell_job_expressions.solve_for_x_so_that_root_expression_matches()
    );
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Minus,
    Multiplication,
    Division,
}

impl Operation {
    fn apply(self, left: isize, right: isize) -> Result<isize, String> {
        match self {
            Operation::Plus => Ok(left + right),
            Operation::Minus => Ok(left - right),
            Operation::Multiplication => Ok(left * right),
            Operation::Division => {
                if right == 0 {
                    Err("Division by zero".to_string())
                } else if left % right != 0 {
                    Err(format!(
                        "Can't divide {} by {} without remainder",
                        left, right
                    ))
                } else {
                    Ok(left / right)
                }
            }
        }
    }

    fn opposite(self) -> Operation {
        match self {
            Operation::Plus => Operation::Minus,
            Operation::Minus => Operation::Plus,
            Operation::Multiplication => Operation::Division,
            Operation::Division => Operation::Multiplication,
        }
    }
}

#[derive(Debug)]
enum YellJob<'a> {
    Constant(isize),
    BinaryExpression {
        operation: Operation,
        left_monkey: &'a str,
        right_monkey: &'a str,
    },
}

fn parse_jobs<'a>(lines: &'a Vec<String>) -> Result<HashMap<&'a str, YellJob<'a>>, String> {
    lines.iter().map(|l| parse_job(l)).collect()
}

fn parse_job<'a>(line: &'a str) -> Result<(&'a str, YellJob<'a>), String> {
    lazy_static! {
        static ref CONSTANT_JOB_RE: Regex = Regex::new(r"^([a-z]+): (\d+)$").unwrap();
        static ref BINARY_EXPRESSION_JOB_RE: Regex =
            Regex::new(r"^([a-z]+): ([a-z]+) (\+|\*|/|\-) ([a-z]+)$").unwrap();
    }
    if let Some(captures) = CONSTANT_JOB_RE.captures(line) {
        return match (
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str().parse(),
        ) {
            (monkey, Ok(constant_value)) => Ok((monkey, YellJob::Constant(constant_value))),
            _ => Err(format!("Can't parse monkey job out of '{}'", line)),
        };
    };
    if let Some(captures) = BINARY_EXPRESSION_JOB_RE.captures(line) {
        let (monkey, left_monkey, operation_str, right_monkey) = (
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str(),
            captures.get(3).unwrap().as_str(),
            captures.get(4).unwrap().as_str(),
        );
        let operation = match operation_str {
            "+" => Operation::Plus,
            "-" => Operation::Minus,
            "*" => Operation::Multiplication,
            "/" => Operation::Division,
            _ => {
                return Err(format!(
                    "Can't parse monkey job operation out of '{}', whole line: {}",
                    operation_str, line
                ))
            }
        };
        return Ok((
            monkey,
            YellJob::BinaryExpression {
                left_monkey,
                operation,
                right_monkey,
            },
        ));
    };
    Err(format!("Can't parse monkey job out of '{}'", line))
}

#[derive(Clone)]
enum Expression {
    Constant(isize),
    X,
    OperationAppliedToExpressionAndConstant {
        expression_with_x: Box<Expression>,
        operation: Operation,
        constant_value: isize,
    },
    OperationAppliedToConstantAndExpression {
        constant_value: isize,
        operation: Operation,
        expression_with_x: Box<Expression>,
    },
}

impl Expression {
    fn value_if_x_is(&self, x_value: isize) -> Result<isize, String> {
        match self {
            Expression::Constant(constant_value) => Ok(*constant_value),
            Expression::X => Ok(x_value),
            Expression::OperationAppliedToExpressionAndConstant {
                expression_with_x,
                operation,
                constant_value,
            } => operation.apply(expression_with_x.value_if_x_is(x_value)?, *constant_value),
            Expression::OperationAppliedToConstantAndExpression {
                constant_value,
                operation,
                expression_with_x,
            } => operation.apply(*constant_value, expression_with_x.value_if_x_is(x_value)?),
        }
    }
}

struct YellJobExpressions {
    root_expression: Expression,
    x_value_as_in_first_part: isize,
}

impl YellJobExpressions {
    fn new(jobs: HashMap<&str, YellJob>) -> Result<YellJobExpressions, String> {
        let x_value_as_in_first_part = match jobs.get("humn") {
            Some(YellJob::Constant(value)) => *value,
            _ => return Err("Expected humn monkey to have a constant job".to_string()),
        };

        let expressions_per_monkey = Self::calculate_expressions_per_monkey(jobs)?;

        let root_expression = match expressions_per_monkey.get("root") {
            Some(expression) => expression,
            _ => return Err("Couldn't find root monkey expression".to_string()),
        };

        Ok(YellJobExpressions {
            root_expression: root_expression.clone(),
            x_value_as_in_first_part,
        })
    }

    fn calculate_expressions_per_monkey<'a>(
        jobs: HashMap<&'a str, YellJob<'a>>,
    ) -> Result<HashMap<&'a str, Expression>, String> {
        let mut calculated = HashMap::new();
        let mut stack = vec![];

        stack.push("root");

        while let Some(monkey) = stack.pop() {
            if monkey == "humn" {
                calculated.insert(monkey, Expression::X);
                continue;
            }
            let job = match jobs.get(monkey) {
                Some(job) => job,
                None => return Err(format!("Couldn't find a job for monkey {}", monkey)),
            };
            match job {
                YellJob::Constant(value) => {
                    calculated.insert(monkey, Expression::Constant(*value));
                }
                YellJob::BinaryExpression {
                    operation: yell_job_operation,
                    left_monkey,
                    right_monkey,
                } => match (calculated.get(left_monkey), calculated.get(right_monkey)) {
                    (None, None) => {
                        stack.push(monkey);
                        stack.push(&left_monkey);
                        stack.push(&right_monkey);
                    }
                    (None, Some(_)) => {
                        stack.push(monkey);
                        stack.push(&left_monkey);
                    }
                    (Some(_), None) => {
                        stack.push(monkey);
                        stack.push(&right_monkey);
                    }
                    (Some(left_expression), Some(right_expression)) => {
                        calculated.insert(
                            monkey,
                            Self::calculate_expression(
                                left_expression,
                                right_expression,
                                *yell_job_operation,
                            )?,
                        );
                    }
                },
            };
        }

        Ok(calculated)
    }

    fn calculate_expression(
        left_expression: &Expression,
        right_expression: &Expression,
        yell_job_operation: Operation,
    ) -> Result<Expression, String> {
        let expression = match (left_expression, right_expression) {
            (
                Expression::Constant(left_constant_value),
                Expression::Constant(right_constant_value),
            ) => Expression::Constant(
                yell_job_operation.apply(*left_constant_value, *right_constant_value)?,
            ),
            (Expression::Constant(constant), Expression::X) => {
                Expression::OperationAppliedToConstantAndExpression {
                    constant_value: *constant,
                    operation: yell_job_operation,
                    expression_with_x: Box::new(Expression::X),
                }
            }
            (
                Expression::Constant(left_constant_value),
                right_expression_with_x @ Expression::OperationAppliedToExpressionAndConstant {
                    expression_with_x: _,
                    operation: _,
                    constant_value: _,
                },
            ) => Expression::OperationAppliedToConstantAndExpression {
                constant_value: *left_constant_value,
                operation: yell_job_operation,
                expression_with_x: Box::new(right_expression_with_x.clone()),
            },
            (
                Expression::Constant(left_constant_value),
                right_expression_with_x @ Expression::OperationAppliedToConstantAndExpression {
                    constant_value: _,
                    operation: _,
                    expression_with_x: _,
                },
            ) => Expression::OperationAppliedToConstantAndExpression {
                constant_value: *left_constant_value,
                operation: yell_job_operation,
                expression_with_x: Box::new(right_expression_with_x.clone()),
            },
            (Expression::X, Expression::Constant(right_constant_value)) => {
                Expression::OperationAppliedToExpressionAndConstant {
                    expression_with_x: Box::new(Expression::X),
                    operation: yell_job_operation,
                    constant_value: *right_constant_value,
                }
            }
            (
                left_expression_with_x @ Expression::OperationAppliedToExpressionAndConstant {
                    expression_with_x: _,
                    operation: _,
                    constant_value: _,
                },
                Expression::Constant(right_constant_value),
            ) => Expression::OperationAppliedToExpressionAndConstant {
                expression_with_x: Box::new(left_expression_with_x.clone()),
                operation: yell_job_operation,
                constant_value: *right_constant_value,
            },
            (
                left_expression_with_x @ Expression::OperationAppliedToConstantAndExpression {
                    constant_value: _,
                    operation: _,
                    expression_with_x: _,
                },
                Expression::Constant(right_constant_value),
            ) => Expression::OperationAppliedToExpressionAndConstant {
                expression_with_x: Box::new(left_expression_with_x.clone()),
                operation: yell_job_operation,
                constant_value: *right_constant_value,
            },
            _ => return Err("Monkey can't have x in both parts of binary expression".to_string()),
        };
        Ok(expression)
    }

    fn root_expression_value_as_in_first_part(&self) -> Result<isize, String> {
        self.root_expression
            .value_if_x_is(self.x_value_as_in_first_part)
    }

    fn solve_for_x_so_that_root_expression_matches(&self) -> Result<isize, String> {
        match &self.root_expression {
            Expression::OperationAppliedToExpressionAndConstant {
                expression_with_x,
                operation: _,
                constant_value,
            } => Self::solve_for_x(expression_with_x, *constant_value),
            Expression::OperationAppliedToConstantAndExpression {
                constant_value,
                operation: _,
                expression_with_x,
            } => Self::solve_for_x(expression_with_x, *constant_value),
            Expression::Constant(_) => return Err("Expected root expression to be binary, where one operand contains x: found constant".to_string()),
            Expression::X => return Err("Expected root expression to be binary, where one operand contains x: found x".to_string()),
        }
    }

    fn solve_for_x(
        expression_with_x: &Expression,
        expression_value: isize,
    ) -> Result<isize, String> {
        let mut current_expression = expression_with_x;
        let mut current_value = expression_value;
        loop {
            match &current_expression {
                Expression::X => return Ok(current_value),
                Expression::OperationAppliedToExpressionAndConstant {
                    expression_with_x,
                    operation,
                    constant_value,
                } => {
                    current_expression = expression_with_x;
                    current_value = operation.opposite().apply(current_value, *constant_value)?;
                }
                Expression::OperationAppliedToConstantAndExpression {
                    constant_value,
                    operation,
                    expression_with_x,
                } => match operation {
                    Operation::Plus => {
                        current_expression = expression_with_x;
                        current_value = Operation::Minus.apply(current_value, *constant_value)?;
                    }
                    Operation::Minus => {
                        current_expression = expression_with_x;
                        current_value = Operation::Minus.apply(*constant_value, current_value)?;
                    }
                    Operation::Multiplication => {
                        current_expression = expression_with_x;
                        current_value =
                            Operation::Division.apply(current_value, *constant_value)?;
                    }
                    Operation::Division => {
                        current_expression = expression_with_x;
                        current_value =
                            Operation::Division.apply(*constant_value, current_value)?;
                    }
                },
                Expression::Constant(_) => panic!("This shouldn't happen"),
            }
        }
    }
}
