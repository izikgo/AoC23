use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Clone, Copy, Debug)]
struct PartNumber {
    value: u32,
    row: u32,
    start_col: u32,
    end_col: u32,
}

// struct containing the data for the problem
// 2d char array
struct DataMatrix {
    data: Vec<String>,
    rows: u32,
    cols: u32,
}

impl DataMatrix {
    fn get(&self, row: u32, col: u32) -> Option<char> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        self.data[row as usize].chars().nth(col as usize)
    }

    fn get_gear_ratio(&self, row: u32, col: u32, adjacent_part_numbers: &Vec<PartNumber>) -> u32 {
        let adj_parts = self.get_adjacent_part_numbers(row, col, adjacent_part_numbers);
        if adj_parts.len() != 2 {
            return 0;
        }
        let ratio = adj_parts[0].value * adj_parts[1].value;
        ratio
    }

    fn get_adjacent_part_numbers(
        &self,
        row: u32,
        col: u32,
        part_numbers: &Vec<PartNumber>,
    ) -> Vec<PartNumber> {
        let mut adjacent_part_numbers: Vec<PartNumber> = Vec::new();
        for part_number in part_numbers {
            if part_number.row + 1 >= row
                && row as i32 >= part_number.row as i32 - 1
                && part_number.start_col as i32 - 1 <= col as i32
                && part_number.end_col + 1 >= col
            {
                adjacent_part_numbers.push(*part_number);
            }
        }
        adjacent_part_numbers
    }

    fn get_all_part_numbers(&self) -> Vec<PartNumber> {
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let re = Regex::new(r"[0-9]+").unwrap();
        for (row, line) in self.data.iter().enumerate() {
            for m in re.find_iter(line) {
                let value = m.as_str().parse::<u32>().unwrap();
                let start_col = m.start() as u32;
                let end_col = m.end() as u32 - 1;
                part_numbers.push(PartNumber {
                    value,
                    row: row as u32,
                    start_col,
                    end_col,
                });
            }
        }
        part_numbers
    }

    fn is_part_number_adjacent_to_symbol(&self, part_number: &PartNumber) -> bool {
        let row_above = std::cmp::max(0, part_number.row as i32 - 1) as u32;
        let row_below = std::cmp::min(self.rows - 1, part_number.row + 1);
        let col_left = std::cmp::max(0, part_number.start_col as i32 - 1) as u32;
        let col_right = std::cmp::min(self.cols - 1, part_number.end_col + 1);

        for row in row_above..=row_below {
            for col in col_left..=col_right {
                let cell_val = self.get(row, col).unwrap();
                if !cell_val.is_digit(10) && cell_val != '.' {
                    return true;
                }
            }
        }
        false
    }

    fn get_sum_of_part_numbers_adjacent_to_symbol(&self) -> u32 {
        let part_numbers = self.get_all_part_numbers();
        let mut sum = 0;
        for part_number in part_numbers {
            if self.is_part_number_adjacent_to_symbol(&part_number) {
                sum += part_number.value;
            }
        }
        sum
    }

    fn get_sum_of_gear_ratios(&self) -> u32 {
        let mut sum = 0;
        let part_numbers = self.get_all_part_numbers();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_val = self.get(row, col).unwrap();
                if cell_val == '*' {
                    let adjacent_part_numbers =
                        self.get_adjacent_part_numbers(row, col, &part_numbers);
                    let gear_ratio = self.get_gear_ratio(row, col, &adjacent_part_numbers);
                    sum += gear_ratio;
                }
            }
        }
        sum
    }
}

impl FromStr for DataMatrix {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let rows = data.len() as u32;
        let cols = data[0].len() as u32;
        Ok(DataMatrix { data, rows, cols })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data_matrix = DataMatrix::from_str(input).unwrap();
    Some(data_matrix.get_sum_of_part_numbers_adjacent_to_symbol())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data_matrix = DataMatrix::from_str(input).unwrap();
    Some(data_matrix.get_sum_of_gear_ratios())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
