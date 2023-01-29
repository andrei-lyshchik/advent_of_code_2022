use std::{collections::HashMap, ops};

use aoc2022::input::read_lines;

fn main() {
    let mut numbers = match read_lines("inputs/day20.txt")
        .map(|l| l.parse())
        .collect::<Result<Vec<isize>, _>>()
        .map(|numbers| Numbers::new(numbers))
    {
        Ok(numbers) => numbers,
        Err(err) => {
            println!("Can't parse numbers: {}", err);
            return;
        }
    };
    let mut multiplied_numbers = &numbers * 811589153;

    numbers.mix_all();

    println!(
        "Part 1: {}",
        numbers.sum_by_offsets_from_0(vec![1000, 2000, 3000])
    );
    for _ in 0..10 {
        multiplied_numbers.mix_all();
    }
    println!(
        "Part 2: {}",
        multiplied_numbers.sum_by_offsets_from_0(vec![1000, 2000, 3000])
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct IndexedNumber {
    value: isize,
    original_index: usize,
}

impl ops::Mul<isize> for &IndexedNumber {
    type Output = IndexedNumber;

    fn mul(self, multiplier: isize) -> IndexedNumber {
        IndexedNumber {
            value: self.value * multiplier,
            original_index: self.original_index,
        }
    }
}

struct Numbers {
    original_numbers_order: Vec<isize>,
    indexes: HashMap<IndexedNumber, usize>,
    numbers: HashMap<usize, IndexedNumber>,
    mixed: usize,
    original_index_of_0: usize,
}

impl Numbers {
    fn new(numbers: Vec<isize>) -> Numbers {
        let mut indexes_per_number = HashMap::new();
        let mut numbers_per_index = HashMap::new();
        let mut original_index_of_0 = 0;

        for (index, number) in numbers.iter().enumerate() {
            let indexed_number = IndexedNumber {
                value: *number,
                original_index: index,
            };
            indexes_per_number.insert(indexed_number, index);
            numbers_per_index.insert(index, indexed_number);
            if *number == 0 {
                original_index_of_0 = index;
            }
        }

        Numbers {
            original_numbers_order: numbers,
            indexes: indexes_per_number,
            numbers: numbers_per_index,
            mixed: 0,
            original_index_of_0,
        }
    }

    fn sum_by_offsets_from_0(&self, indexes: Vec<usize>) -> isize {
        let mut result = 0;
        let current_index_of_0 = self
            .indexes
            .get(&IndexedNumber {
                value: 0,
                original_index: self.original_index_of_0,
            })
            .unwrap();
        for offset_from_0 in &indexes {
            result += self.numbers[&((current_index_of_0 + offset_from_0) % self.len())].value;
        }

        result
    }

    fn mix_all(&mut self) {
        for _ in 0..self.len() {
            self.mix_single_number();
        }
        self.mixed = 0;
    }

    fn mix_single_number(&mut self) {
        let number = IndexedNumber {
            value: self.original_numbers_order[self.mixed],
            original_index: self.mixed,
        };
        if number.value == 0 {
            self.mixed += 1;
            return;
        }
        let current_index = *self.indexes.get(&number).unwrap();
        let (current_index, shift_by) = if current_index == 0 {
            let number_at_1 = *self.numbers.get(&1).unwrap();
            self.indexes.insert(number_at_1, 0);
            self.numbers.insert(0, number_at_1);
            self.indexes.insert(number, 1);
            self.numbers.insert(1, number);
            (current_index + 1, number.value - 1)
        } else {
            (current_index, number.value)
        };

        self.shift(number, current_index, shift_by);
        self.mixed += 1;
    }

    fn shift(&mut self, number: IndexedNumber, current_index: usize, shift_by: isize) {
        let shift_by_normalized = shift_by % (self.len() as isize - 1);
        let shift_by_normalized = if shift_by_normalized >= 0 {
            shift_by_normalized as usize
        } else {
            (shift_by_normalized + (self.len() as isize - 1)) as usize
        };
        if shift_by_normalized == 0 {
            return;
        }

        let target_index = (current_index + shift_by_normalized) % self.len()
            + (current_index + shift_by_normalized) / self.len();
        if target_index > current_index {
            for index in (current_index + 1)..=target_index {
                let number_to_shift_back = self.numbers.get(&index).unwrap();
                self.indexes.insert(*number_to_shift_back, index - 1);
                self.numbers.insert(index - 1, *number_to_shift_back);
            }
        } else if target_index < current_index {
            for index in (target_index..=(current_index - 1)).rev() {
                let number_to_shift_forward = self.numbers.get(&index).unwrap();
                self.indexes.insert(*number_to_shift_forward, index + 1);
                self.numbers.insert(index + 1, *number_to_shift_forward);
            }
        }
        self.indexes.insert(number, target_index);
        self.numbers.insert(target_index, number);
    }

    fn len(&self) -> usize {
        self.original_numbers_order.len()
    }

    fn current_order(&self) -> Vec<isize> {
        let mut result = vec![];

        for index in 0..self.len() {
            result.push(self.numbers.get(&index).unwrap().value);
        }

        result
    }
}

impl ops::Mul<isize> for &Numbers {
    type Output = Numbers;

    fn mul(self, multiplier: isize) -> Numbers {
        let original_numbers_order = self
            .original_numbers_order
            .iter()
            .map(|n| n * multiplier)
            .collect();
        let mut indexes = HashMap::new();
        for (number, index) in self.indexes.iter() {
            indexes.insert(number * multiplier, *index);
        }
        let mut numbers = HashMap::new();
        for (index, number) in self.numbers.iter() {
            numbers.insert(*index, number * multiplier);
        }
        Numbers {
            original_numbers_order,
            indexes,
            numbers,
            original_index_of_0: self.original_index_of_0,
            mixed: self.mixed,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Numbers;

    #[test]
    fn examples_from_description() {
        let mut numbers = Numbers::new(vec![1, 2, -3, 3, -2, 0, 4]);

        assert_eq!(vec![1, 2, -3, 3, -2, 0, 4], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![2, 1, -3, 3, -2, 0, 4], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![1, -3, 2, 3, -2, 0, 4], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![1, 2, 3, -2, -3, 0, 4], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![1, 2, -2, -3, 0, 3, 4], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![1, 2, -3, 0, 3, 4, -2], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![1, 2, -3, 0, 3, 4, -2], numbers.current_order());

        numbers.mix_single_number();
        assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], numbers.current_order());

        assert_eq!(3, numbers.sum_by_offsets_from_0(vec![1000, 2000, 3000]));
    }
}
