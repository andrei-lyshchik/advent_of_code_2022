use aoc2022::input::read_lines;

fn main() {
    let strategies: Vec<GameStrategy> = read_lines("inputs/day2.txt")
        .map(|l| GameStrategy::parse(&l).unwrap())
        .collect();

    println!(
        "Part 1: outcome {}",
        strategies.iter().map(|s| s.score_part1()).sum::<i32>()
    );
    println!(
        "Part 2: outcome {}",
        strategies.iter().map(|s| s.score_part2()).sum::<i32>()
    );
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self: &Self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(self: &Self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn calculate_outcome(opponent: &Shape, you: &Shape) -> Self {
        match (opponent, you) {
            (Shape::Rock, Shape::Rock) => Self::Draw,
            (Shape::Rock, Shape::Paper) => Self::Win,
            (Shape::Rock, Shape::Scissors) => Self::Loss,
            (Shape::Paper, Shape::Rock) => Self::Loss,
            (Shape::Paper, Shape::Paper) => Self::Draw,
            (Shape::Paper, Shape::Scissors) => Self::Win,
            (Shape::Scissors, Shape::Rock) => Self::Win,
            (Shape::Scissors, Shape::Paper) => Self::Loss,
            (Shape::Scissors, Shape::Scissors) => Self::Draw,
        }
    }

    fn calculate_necessary_shape(opponent: &Shape, desired_outcome: &Self) -> Shape {
        match (opponent, desired_outcome) {
            (Shape::Rock, Self::Draw) => Shape::Rock,
            (Shape::Rock, Self::Win) => Shape::Paper,
            (Shape::Rock, Self::Loss) => Shape::Scissors,
            (Shape::Paper, Self::Loss) => Shape::Rock,
            (Shape::Paper, Self::Draw) => Shape::Paper,
            (Shape::Paper, Self::Win) => Shape::Scissors,
            (Shape::Scissors, Self::Win) => Shape::Rock,
            (Shape::Scissors, Self::Loss) => Shape::Paper,
            (Shape::Scissors, Self::Draw) => Shape::Scissors,
        }
    }
}

struct GameStrategy {
    opponent: Shape,
    second_column_as_shape: Shape,
    second_column_as_outcome: Outcome,
}

impl GameStrategy {
    fn score_part1(self: &Self) -> i32 {
        Outcome::calculate_outcome(&self.opponent, &self.second_column_as_shape).score()
            + self.second_column_as_shape.score()
    }

    fn score_part2(self: &Self) -> i32 {
        let necessary_shape =
            Outcome::calculate_necessary_shape(&self.opponent, &self.second_column_as_outcome);
        self.second_column_as_outcome.score() + necessary_shape.score()
    }

    fn parse(line: &str) -> Option<GameStrategy> {
        if line.len() != 3 {
            return None;
        }

        let opponent = match line.chars().nth(0).unwrap() {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => return None,
        };
        let second_column_as_shape = match line.chars().nth(2).unwrap() {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => return None,
        };
        let second_column_as_outcome = match line.chars().nth(2).unwrap() {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => return None,
        };

        Some(GameStrategy {
            opponent,
            second_column_as_shape,
            second_column_as_outcome,
        })
    }
}
