use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(5);

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CategoryMapEntry {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

struct CategoryMapping {
    source: Category,
    target: Category,
    entries: BTreeMap<u64, CategoryMapEntry>,
}

impl CategoryMapping {
    fn get_target_value(&self, source_value: u64) -> u64 {
        // get minimum key
        if source_value < *self.entries.keys().next().unwrap() {
            return source_value;
        }
        let closest_mapping = self.entries.range(..=source_value).last().unwrap().1;
        if source_value < closest_mapping.source_range_start + closest_mapping.range_length {
            return closest_mapping.destination_range_start
                + (source_value - closest_mapping.source_range_start);
        }
        source_value
    }
}

impl FromStr for CategoryMapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let cap = Regex::new(r"([a-z]+)-to-([a-z]+) map:")
            .unwrap()
            .captures(lines.next().unwrap())
            .unwrap();
        let (source, target) = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
        let mut entries = BTreeMap::new();
        for line in lines {
            let (dest, src, len) = line.split(" ").collect_tuple().unwrap();
            entries.insert(
                src.parse().unwrap(),
                CategoryMapEntry {
                    source_range_start: src.parse().unwrap(),
                    destination_range_start: dest.parse().unwrap(),
                    range_length: len.parse().unwrap(),
                },
            );
        }
        Ok(CategoryMapping {
            source,
            target,
            entries,
        })
    }
}

struct Almanac {
    seeds: Vec<u64>,
    mappings: HashMap<(Category, Category), CategoryMapping>,
    category_map: HashMap<Category, Category>,
}

impl Almanac {
    fn get_seed_location(&self, seed: u64) -> u64 {
        let mut cur_source_category = Category::Seed;
        let mut cur_source_value = seed;
        while cur_source_category != Category::Location {
            let cur_dest_category = self.category_map[&cur_source_category];
            let mapping = self
                .mappings
                .get(&(cur_source_category, cur_dest_category))
                .unwrap();
            cur_source_value = mapping.get_target_value(cur_source_value);
            cur_source_category = mapping.target;
        }
        cur_source_value
    }

    fn get_lowest_seed_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_location(*seed))
            .min()
            .unwrap()
    }

    fn get_lowest_seed_location_ranges(&self) -> u64 {
        // This is v
        let min_all_seeds = self
            .seeds
            .chunks(2)
            .map(|c| c[0]..(c[0] + c[1]))
            .flatten()
            .map(|seed| self.get_seed_location(seed))
            .min()
            .unwrap();
        min_all_seeds
    }
}
impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n\n");
        let re = Regex::new(r"\d+").unwrap();
        let seeds = re
            .find_iter(sections.next().unwrap())
            .map(|m| m.as_str().parse().unwrap())
            .collect::<Vec<u64>>();
        let mut mappings = HashMap::new();
        let mut category_map = HashMap::new();
        for section in sections {
            let mapping = section.parse::<CategoryMapping>().unwrap();
            let source = mapping.source.clone();
            let target = mapping.target.clone();
            mappings.insert((mapping.source, mapping.target), mapping);
            category_map.insert(source, target);
        }
        Ok(Almanac {
            seeds,
            mappings,
            category_map,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = input.parse::<Almanac>().unwrap();
    Some(almanac.get_lowest_seed_location())
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = input.parse::<Almanac>().unwrap();
    Some(almanac.get_lowest_seed_location_ranges())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
