use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        // find the first digit
        let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
        // find the last digit
        let last_digit = line.chars().rfind(|c| c.is_digit(10)).unwrap();
        // concat the digits
        let digits = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();
        // add to total
        total += digits;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let digit_map: HashMap<_, _> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let pattern_str = r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9";
    let pattern = Regex::new(pattern_str).unwrap();
    let pattern_reversed = Regex::new(&pattern_str.chars().rev().collect::<String>()).unwrap();

    let mut total = 0;
    for line in input.lines() {
        // find first digit from pattern
        let first_digit_str = pattern.find(line).unwrap().as_str();
        let first_digit = *digit_map.get(first_digit_str).unwrap_or(&first_digit_str);

        // find last digit from pattern
        // we use reversed string and pattern since there are overlapping matches
        let line_reversed = line.chars().rev().collect::<String>();
        let last_digit_str_rev = pattern_reversed.find(&line_reversed).unwrap().as_str();
        let last_digit_str = last_digit_str_rev.chars().rev().collect::<String>();
        let last_digit = *digit_map
            .get(last_digit_str.as_str())
            .unwrap_or(&last_digit_str.as_str());

        // concat the digits
        let digits = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();

        // add to total
        total += digits;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result.unwrap(), 281);
    }
}
