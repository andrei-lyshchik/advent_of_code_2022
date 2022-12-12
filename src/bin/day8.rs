use aoc2022::input::read_lines;

fn main() {
    let grid_value = read_lines("inputs/day8.txt")
        .map(|l| l.chars().map(|c| (c as u8 - b'0') as i8).collect())
        .collect();

    if let Some(grid) = Grid::new(grid_value) {
        println!(
            "Part 1: number of visible trees: {}",
            grid.calculate_visible_trees()
        );

        println!("Part 2: max scenic index: {}", grid.max_scenic_index())
    }
}

struct Grid {
    value: Vec<Vec<i8>>,
    height: usize,
    length: usize,
}

impl Grid {
    fn new(vec: Vec<Vec<i8>>) -> Option<Grid> {
        if vec.is_empty() {
            return None;
        }
        let height = vec.len();
        if vec[0].is_empty() {
            return None;
        }
        let length = vec[0].len();
        Some(Grid {
            value: vec,
            height,
            length,
        })
    }

    fn calculate_visible_trees(self: &Self) -> usize {
        let maximums = self.calculate_maximums();
        let mut result = 0;
        for row in 0..self.height {
            for col in 0..self.length {
                if maximums.iter().any(|m| m[row][col] < self.value[row][col]) {
                    result += 1;
                }
            }
        }
        result
    }

    fn initialize_maximum(self: &Self) -> Vec<Vec<i8>> {
        vec![vec![-1; self.length]; self.height]
    }

    fn calculate_maximums(self: &Self) -> [Vec<Vec<i8>>; 4] {
        let mut top_to_bottom = self.initialize_maximum();
        for col in 0..self.length {
            top_to_bottom[1][col] = self.value[0][col];
            for row in 2..self.height {
                top_to_bottom[row][col] =
                    i8::max(top_to_bottom[row - 1][col], self.value[row - 1][col]);
            }
        }
        let mut bottom_to_top = self.initialize_maximum();
        for col in 0..self.length {
            bottom_to_top[self.height - 2][col] = self.value[self.height - 1][col];
            for row in (0..self.height - 2).rev() {
                bottom_to_top[row][col] =
                    i8::max(bottom_to_top[row + 1][col], self.value[row + 1][col]);
            }
        }
        let mut left_to_right = self.initialize_maximum();
        for row in 0..self.height {
            left_to_right[row][1] = self.value[row][0];
            for col in 2..self.length {
                left_to_right[row][col] =
                    i8::max(left_to_right[row][col - 1], self.value[row][col - 1]);
            }
        }
        let mut right_to_left = self.initialize_maximum();
        for row in 0..self.height {
            right_to_left[row][self.height - 2] = self.value[row][self.height - 1];
            for col in (0..self.length - 2).rev() {
                right_to_left[row][col] =
                    i8::max(right_to_left[row][col + 1], self.value[row][col + 1]);
            }
        }
        [top_to_bottom, bottom_to_top, left_to_right, right_to_left]
    }

    fn max_scenic_index(self: &Self) -> usize {
        (1..self.height - 1)
            .flat_map(move |row| (1..self.length - 1).map(move |col| self.scenic_index(row, col)))
            .max()
            .unwrap()
    }

    fn scenic_index(self: &Self, row: usize, col: usize) -> usize {
        let value_at_row_col = self.value[row][col];
        let mut up_visible = 0;
        for i in (0..row).rev() {
            up_visible += 1;
            if self.value[i][col] >= value_at_row_col {
                break;
            }
        }
        let mut left_visible = 0;
        for i in (0..col).rev() {
            left_visible += 1;
            if self.value[row][i] >= value_at_row_col {
                break;
            }
        }
        let mut down_visible = 0;
        for i in row + 1..self.height {
            down_visible += 1;
            if self.value[i][col] >= value_at_row_col {
                break;
            }
        }
        let mut right_visible = 0;
        for i in col + 1..self.length {
            right_visible += 1;
            if self.value[row][i] >= value_at_row_col {
                break;
            }
        }
        up_visible * left_visible * down_visible * right_visible
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn test_calculate_visible_trees() {
        let grid = Grid::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();

        assert_eq!(21, grid.calculate_visible_trees());
    }

    #[test]
    fn test_max_scenic_index() {
        let grid = Grid::new(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();

        assert_eq!(8, grid.max_scenic_index());
    }
}
