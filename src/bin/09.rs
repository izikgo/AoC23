use std::str::FromStr;

advent_of_code::solution!(9);

struct ReportLine {
    values: Vec<i64>,
}

impl FromStr for ReportLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().expect("Failed to parse number"))
            .collect::<Vec<_>>();
        Ok(ReportLine { values })
    }
}

impl ReportLine {
    fn build_grid(&self) -> Vec<Vec<i64>> {
        let mut grid = vec![vec![0; self.values.len()]; self.values.len()];
        // fill last row
        for j in 0..self.values.len() {
            grid.last_mut().unwrap()[j] = self.values[j];
        }

        for i in (0..grid.len() - 1).rev() {
            for j in 0..=i {
                let val = grid[i + 1][j + 1] - grid[i + 1][j];
                grid[i][j] = val;
            }
        }

        grid
    }

    fn predict_next(&self) -> i64 {
        let grid = self.build_grid();

        let res = (0..grid.len()).map(|i| grid[i][i]).sum();
        res
    }

    fn predict_previous(&self) -> i64 {
        let grid = self.build_grid();

        let res = (0..grid.len()).fold(0, |acc, i| grid[i][0] - acc);
        res
    }
}

struct Report {
    lines: Vec<ReportLine>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| line.parse::<ReportLine>().expect("Failed to parse line"))
            .collect::<Vec<_>>();
        Ok(Report { lines })
    }
}

impl Report {
    fn get_sum_next(&self) -> i64 {
        self.lines.iter().map(|line| line.predict_next()).sum()
    }

    fn get_sum_previous(&self) -> i64 {
        self.lines.iter().map(|line| line.predict_previous()).sum()
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let report = input.parse::<Report>().expect("Failed to parse report");
    Some(report.get_sum_next())
}

pub fn part_two(input: &str) -> Option<i64> {
    let report = input.parse::<Report>().expect("Failed to parse report");
    Some(report.get_sum_previous())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
