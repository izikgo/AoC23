use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(6);

struct Race {
    duration: u64,
    record_distance: u64,
}

impl Race {
    fn get_num_options_to_beat_record(&self) -> u64 {
        // distance traveled is time_pressed * (duration - time_pressed) = time_pressed * duration - time_pressed^2
        // we need to find the 0 intercept of the quadratic equation
        // 0 = time_pressed * duration - time_pressed^2 - record_distance
        // 0 = time_pressed^2 - time_pressed * duration + record_distance
        // time_pressed = (duration +/- sqrt(duration^2 - 4 * record_distance)) / 2

        let duration = self.duration as i64;
        let record_distance = self.record_distance as i64;

        let discriminant = duration.pow(2) - 4 * record_distance;
        if discriminant < 0 {
            0
        } else {
            let root = (discriminant as f64).sqrt();
            let time_pressed_1 = ((duration as f64) - root) / 2.0;
            let time_pressed_2 = ((duration as f64) + root) / 2.0;
            (time_pressed_2 - 1.).ceil() as u64 - (time_pressed_1 + 1.).floor() as u64 + 1
        }
    }
}

impl FromStr for Race {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();

        let (duration, record) = lines
            .map(|s| {
                s.split_once(":")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .ok()
            })
            .collect_tuple()
            .unwrap();

        Ok(Self {
            duration: duration.unwrap(),
            record_distance: record.unwrap(),
        })
    }
}

struct Races {
    races: Vec<Race>,
}

impl Races {
    fn get_num_option_prod(&self) -> u64 {
        self.races
            .iter()
            .map(|r| r.get_num_options_to_beat_record())
            .product()
    }
}

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let durations = lines
            .next()
            .and_then(|s| s.strip_prefix("Time: "))
            .expect("Expected 'Time: ' line prefix")
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let record_distance = lines
            .next()
            .and_then(|s| s.strip_prefix("Distance: "))
            .expect("Expected 'Distance: ' line prefix")
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let races = durations
            .zip(record_distance)
            .map(|(d, r)| Race {
                duration: d,
                record_distance: r,
            })
            .collect_vec();

        Ok(Self { races })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = input.parse::<Races>().unwrap();
    Some(races.get_num_option_prod())
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = input.parse::<Race>().unwrap();
    Some(race.get_num_options_to_beat_record())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
