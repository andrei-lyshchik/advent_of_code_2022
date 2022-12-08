use std::{cell::RefCell, collections::HashMap, rc::Rc};

use aoc2022::input::read_lines;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn main() {
    let lines = read_lines("inputs/day7.txt").collect::<Vec<_>>();

    if let Ok(commands_with_output) = CommandWithOutput::parse_list(&lines) {
        if let Ok(total_sizes) = calculate_total_sizes_with_child_directories(&commands_with_output)
        {
            let sizes_sum_under_100000: usize = total_sizes.iter().filter(|s| **s <= 100000).sum();
            println!(
                "Part 1: directory sizes sum under 100000: {}",
                sizes_sum_under_100000
            );

            let total_used = total_sizes.iter().max().unwrap();
            let min_size_required_to_free = total_sizes
                .iter()
                .filter(|s| **s >= *total_used - 40000000)
                .min();
            println!(
                "Part 2: min directory size required to free: {:?}",
                min_size_required_to_free,
            );
            return;
        }
    }
    println!("Error with commands output");
}

fn calculate_total_sizes_with_child_directories(
    commands_with_output: &Vec<CommandWithOutput>,
) -> Result<Vec<usize>, String> {
    DirectoryTreeNode::build_tree(commands_with_output)
        .map(|root| DirectoryTreeNode::calculate_total_sizes_with_child_directories(root))
}

#[derive(Debug, PartialEq)]
struct DirectoryTreeNode<'a> {
    child_files_size: usize,
    total_size_with_child_directories: usize,
    child_directories: HashMap<&'a str, Rc<RefCell<DirectoryTreeNode<'a>>>>,
}

impl<'a> DirectoryTreeNode<'a> {
    fn build_tree(
        commands_with_output: &Vec<CommandWithOutput<'a>>,
    ) -> Result<Rc<RefCell<DirectoryTreeNode<'a>>>, String> {
        let root = Rc::new(RefCell::new(Self::empty()));
        let mut current_directory = Rc::clone(&root);
        let mut parents_stack: Vec<Rc<RefCell<DirectoryTreeNode>>> = Vec::new();
        for c in commands_with_output {
            match c {
                CommandWithOutput::ChangeDirectoryToRoot => {
                    current_directory = Rc::clone(&root);
                }
                CommandWithOutput::ChangeDirectoryTo { directory_name } => {
                    let new_current_directory = if let Some(destination_directory_node) =
                        current_directory
                            .as_ref()
                            .borrow()
                            .child_directories
                            .get(directory_name)
                    {
                        Rc::clone(destination_directory_node)
                    } else {
                        return Err(format!("Changing to unknown directory {}", directory_name));
                    };
                    parents_stack.push(Rc::clone(&current_directory));
                    current_directory = new_current_directory;
                }
                CommandWithOutput::ChangeDirectoryToParent => {
                    if let Some(parent) = parents_stack.pop() {
                        current_directory = parent;
                    } else {
                        current_directory = Rc::clone(&root);
                    }
                }
                CommandWithOutput::ListDirectory { output } => {
                    Self::fill_node_from_list_directory_output(
                        Rc::clone(&current_directory),
                        output,
                    );
                }
            }
        }

        Ok(root)
    }

    fn empty() -> DirectoryTreeNode<'a> {
        DirectoryTreeNode {
            child_files_size: 0,
            total_size_with_child_directories: 0,
            child_directories: HashMap::new(),
        }
    }

    fn fill_node_from_list_directory_output(
        node: Rc<RefCell<DirectoryTreeNode<'a>>>,
        output: &Vec<ListDirectoryOutputRow<'a>>,
    ) {
        for output_row in output.iter() {
            match output_row {
                ListDirectoryOutputRow::Directory { name } => {
                    node.borrow_mut()
                        .child_directories
                        .insert(*name, Rc::new(RefCell::new(Self::empty())));
                }
                ListDirectoryOutputRow::File { size } => {
                    node.borrow_mut().child_files_size += size;
                }
            }
        }
    }

    fn calculate_total_sizes_with_child_directories(
        root: Rc<RefCell<DirectoryTreeNode>>,
    ) -> Vec<usize> {
        let mut result = Vec::new();
        let mut stack: Vec<Rc<RefCell<DirectoryTreeNode>>> = Vec::new();

        stack.push(Rc::clone(&root));
        while let Some(directory) = stack.pop() {
            let mut total_size_with_child_directories: usize =
                directory.as_ref().borrow().child_files_size;
            let mut child_directories_without_filled_total_size = vec![];
            for child in directory.as_ref().borrow().child_directories.values() {
                if child
                    .as_ref()
                    .borrow()
                    .total_size_with_child_directories_filled()
                {
                    total_size_with_child_directories +=
                        child.as_ref().borrow().total_size_with_child_directories;
                } else {
                    child_directories_without_filled_total_size.push(Rc::clone(child));
                }
            }
            if child_directories_without_filled_total_size.is_empty() {
                result.push(total_size_with_child_directories);
                directory.borrow_mut().total_size_with_child_directories =
                    total_size_with_child_directories;
            } else {
                stack.push(directory);
                stack.extend(child_directories_without_filled_total_size);
            }
        }

        result
    }

    fn total_size_with_child_directories_filled(self: &DirectoryTreeNode<'a>) -> bool {
        self.total_size_with_child_directories > 0
    }
}

#[derive(Debug, PartialEq)]
enum CommandWithOutput<'a> {
    ChangeDirectoryToRoot,
    ChangeDirectoryTo {
        directory_name: &'a str,
    },
    ChangeDirectoryToParent,
    ListDirectory {
        output: Vec<ListDirectoryOutputRow<'a>>,
    },
}

#[derive(Debug, PartialEq)]
enum ListDirectoryOutputRow<'a> {
    File { size: usize },
    Directory { name: &'a str },
}

impl<'a> CommandWithOutput<'a> {
    fn parse_list(lines: &'a Vec<String>) -> Result<Vec<CommandWithOutput<'a>>, String> {
        let mut lines_iter = lines.iter().peekable();

        let mut result: Vec<CommandWithOutput<'a>> = Vec::new();

        lazy_static! {
            static ref CD_RE: Regex = Regex::new(r"^\$ cd ([a-z./]+)$").unwrap();
        }

        while let Some(line) = lines_iter.next() {
            let command_with_output = if line == "$ ls" {
                let mut output: Vec<ListDirectoryOutputRow> = Vec::new();
                while let Some(&next_line) = lines_iter.peek() {
                    if next_line.starts_with("$") {
                        break;
                    }
                    output.push(Self::parse_list_directory_output_row(next_line)?);
                    lines_iter.next();
                }
                CommandWithOutput::ListDirectory { output }
            } else {
                Self::parse_change_directory(line)?
            };
            result.push(command_with_output);
        }

        Ok(result)
    }

    fn parse_list_directory_output_row(line: &str) -> Result<ListDirectoryOutputRow, String> {
        lazy_static! {
            static ref FILE_REGEX: Regex = Regex::new(r"^(\d+) [a-z.]+$").unwrap();
            static ref DIRECTORY_REGEX: Regex = Regex::new(r"^dir ([a-z.]+)$").unwrap();
        }
        if let Some(file_match) = FILE_REGEX.captures(line) {
            let size = match file_match.get(1).unwrap().as_str().parse::<usize>() {
                Ok(size) => size,
                Err(err) => {
                    return Err(format!(
                        "Can't parse file size out of '{}', error: {}",
                        &file_match[1], err
                    ));
                }
            };
            Ok(ListDirectoryOutputRow::File { size })
        } else if let Some(directory_match) = DIRECTORY_REGEX.captures(&line) {
            Ok(ListDirectoryOutputRow::Directory {
                name: directory_match.get(1).unwrap().as_str(),
            })
        } else {
            return Err(format!("Can't parse ls output line out of '{}'", line));
        }
    }

    fn parse_change_directory(line: &str) -> Result<CommandWithOutput, String> {
        lazy_static! {
            static ref CD_REGEX: Regex = Regex::new(r"^\$ cd ([a-z./]+)$").unwrap();
        }

        if let Some(cd_match) = CD_REGEX.captures(line) {
            let command = match cd_match.get(1).unwrap().as_str() {
                "/" => CommandWithOutput::ChangeDirectoryToRoot,
                ".." => CommandWithOutput::ChangeDirectoryToParent,
                directory_name @ _ => CommandWithOutput::ChangeDirectoryTo { directory_name },
            };
            Ok(command)
        } else {
            Err(format!("Can't parse command out of '{}'", line))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CommandWithOutput, ListDirectoryOutputRow};

    #[test]
    fn test_parsing_commands() {
        let lines = vec![
            "$ cd /".to_owned(),
            "$ cd abscede".to_owned(),
            "$ ls".to_owned(),
            "3242 gldg.jrd".to_owned(),
            "dir qffvbf".to_owned(),
            "$ cd ..".to_owned(),
        ];
        let result = CommandWithOutput::parse_list(&lines);

        assert_eq!(
            result,
            Ok(vec![
                CommandWithOutput::ChangeDirectoryToRoot,
                CommandWithOutput::ChangeDirectoryTo {
                    directory_name: "abscede"
                },
                CommandWithOutput::ListDirectory {
                    output: vec![
                        ListDirectoryOutputRow::File { size: 3242 },
                        ListDirectoryOutputRow::Directory { name: "qffvbf" }
                    ]
                },
                CommandWithOutput::ChangeDirectoryToParent
            ])
        )
    }
}
