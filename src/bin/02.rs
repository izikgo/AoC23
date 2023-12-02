use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(2);
fn get_game_power(line: &str) -> usize {
    let mut game_max_colors_hashmap: HashMap<_, _> =
        HashMap::from_iter([("red", 0), ("green", 0), ("blue", 0)]);

    let (_, outcome) = line.split(": ").collect_tuple().unwrap();

    // split line by ";"
    let draw_split = outcome.split("; ");
    for draw in draw_split {
        // split by ","
        for val_color in draw.split(", ") {
            // split by " "
            let (val, color) = val_color.split(' ').collect_tuple().unwrap();
            let val = val.parse::<usize>().unwrap();

            // update color max value
            game_max_colors_hashmap
                .insert(color, std::cmp::max(val, game_max_colors_hashmap[color]));
        }
    }

    // calculate game power
    game_max_colors_hashmap.values().product()
}

fn is_game_possible(line: &str) -> Option<usize> {
    let max_colors_hashmap: HashMap<&str, usize> =
        HashMap::from_iter([("red", 12), ("green", 13), ("blue", 14)]);
    let max_total: usize = max_colors_hashmap.values().sum();

    let mut game_max_colors_hashmap: HashMap<_, _> =
        HashMap::from_iter([("red", 0), ("green", 0), ("blue", 0)]);

    let (game_name, outcome) = line.split(": ").collect_tuple().unwrap();

    // split line by ";"
    let draw_split = outcome.split("; ");
    for draw in draw_split {
        // split by ","
        for val_color in draw.split(", ") {
            // split by " "
            let (val, color) = val_color.split(' ').collect_tuple().unwrap();
            let val = val.parse::<usize>().unwrap();

            // check if color value is greater than max possible value
            if val > max_colors_hashmap[color] {
                return None;
            }

            // update color max value
            game_max_colors_hashmap
                .insert(color, std::cmp::max(val, game_max_colors_hashmap[color]));
        }

        // check if game is possible by summing max values
        if game_max_colors_hashmap.values().sum::<usize>() > max_total {
            return None;
        }
    }

    // extract game name number
    Some(
        game_name
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    // split input lines
    let games = input.lines();

    // sum all possible games
    Some(
        games
            .filter_map(|line| is_game_possible(line))
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // split input lines
    let games = input.lines();

    // sum all possible games
    Some(
        games
            .map(|line| get_game_power(line))
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 2286);
    }
}
