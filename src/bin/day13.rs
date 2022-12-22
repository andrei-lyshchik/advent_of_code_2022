use std::{cmp::Ordering, iter::Peekable, str::Chars};

use aoc2022::input::read_lines;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DIVIDER_1: Element = Element::List(vec![Element::List(vec![Element::Number(2)])]);
    static ref DIVIDER_2: Element = Element::List(vec![Element::List(vec![Element::Number(6)])]);
}

fn main() {
    let pairs = match parse_pairs(&read_lines("inputs/day13.txt").collect()) {
        Ok(pairs) => pairs,
        Err(err) => {
            println!("Unable to parse pairs: {}", err);
            return;
        }
    };

    println!(
        "Part 1: indices sum where first is less than second: {}",
        pairs
            .iter()
            .enumerate()
            .map(|(i, pair)| if pair.0 < pair.1 { i + 1 } else { 0 })
            .sum::<usize>()
    );

    let mut all_elements = Vec::new();
    for pair in pairs {
        all_elements.push(pair.0);
        all_elements.push(pair.1);
    }
    all_elements.push(DIVIDER_1.clone());
    all_elements.push(DIVIDER_2.clone());
    all_elements.sort();

    println!(
        "Part 2: {:?}",
        all_elements
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if e == &*DIVIDER_1 || e == &*DIVIDER_2 {
                Some(i + 1)
            } else {
                None
            })
            .reduce(|acc, i| acc * i),
    )
}

fn parse_pairs(lines: &Vec<String>) -> Result<Vec<(Element, Element)>, String> {
    let mut line_iter = lines.iter().peekable();
    let mut result = Vec::new();
    while let Some(first) = line_iter.next() {
        let first_element = parse_element(first)?;
        if let Some(second) = line_iter.next() {
            let second_element = parse_element(&second)?;
            result.push((first_element, second_element));
        } else {
            return Err("Expected second line in pair".to_owned());
        }
        if let Some(line) = line_iter.peek() {
            if line.is_empty() {
                line_iter.next();
            }
        }
    }
    Ok(result)
}

#[derive(Debug, Clone)]
enum Element {
    List(Vec<Element>),
    Number(i32),
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(self_children), Self::List(other_children)) => {
                self_children == other_children
            }
            (Self::Number(self_number), Self::Number(other_number)) => self_number == other_number,
            (number @ Self::Number(_), Self::List(other_children)) => {
                other_children.len() == 1 && number == &other_children[0]
            }
            (Self::List(self_children), number @ Self::Number(_)) => {
                self_children.len() == 1 && &self_children[0] == number
            }
        }
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(self_children), Self::List(other_children)) => {
                self_children.cmp(other_children)
            }
            (Self::Number(self_number), Self::Number(other_number)) => {
                self_number.cmp(other_number)
            }
            (Self::Number(number), list @ Self::List(_)) => {
                Self::List(vec![Self::Number(*number)]).cmp(list)
            }
            (list @ Self::List(_), Self::Number(number)) => {
                list.cmp(&Self::List(vec![Self::Number(*number)]))
            }
        }
    }
}

fn parse_element(line: &str) -> Result<Element, String> {
    ElementParser::new(line).parse_whole()
}

struct ElementParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> ElementParser<'a> {
    fn new(line: &'a str) -> ElementParser<'a> {
        ElementParser {
            chars: line.chars().peekable(),
        }
    }

    fn parse_whole(mut self: Self) -> Result<Element, String> {
        let result = self.parse_element()?;
        if let Some(char) = self.chars.next() {
            return Err(format!("Unexpected char: {:?}", char));
        }
        Ok(result)
    }

    fn parse_element(self: &mut Self) -> Result<Element, String> {
        match self.chars.peek() {
            Some('[') => self.parse_list(),
            Some(c) if c.is_digit(10) => self.parse_number(),
            unexpected @ _ => Err(format!("Unexpected char: {:?}", unexpected)),
        }
    }

    fn parse_list(self: &mut Self) -> Result<Element, String> {
        match self.chars.next() {
            Some('[') => {}
            unexpected @ _ => return Err(format!("Unexpected char: {:?}", unexpected)),
        };
        let mut children = Vec::new();
        while let Some(char) = self.chars.peek() {
            if *char == ']' {
                self.chars.next();
                return Ok(Element::List(children));
            }
            children.push(self.parse_element()?);
            match self.chars.peek() {
                Some(',') => {
                    self.chars.next();
                }
                Some(']') => {}
                unexpected @ _ => return Err(format!("Unexpected char: {:?}", unexpected)),
            };
        }
        Err("Unexpected end of list".to_owned())
    }

    fn parse_number(self: &mut Self) -> Result<Element, String> {
        let mut digits = Vec::new();
        while let Some(char) = self.chars.peek().cloned() {
            if char.is_digit(10) {
                digits.push(char);
                self.chars.next();
            } else {
                break;
            }
        }
        match digits.iter().map(|c| *c).collect::<String>().parse() {
            Ok(number) => Ok(Element::Number(number)),
            Err(err) => Err(format!("Couldn't parse number out of {}", err)),
        }
    }
}
