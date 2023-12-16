use num::integer::lcm;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(8);

struct Node {
    left: String,
    right: String,
    name: String,
}

struct Network {
    nodes: HashMap<String, Node>,
    path: String,
}

impl Network {
    fn get_num_steps_to_zzz(&self) -> u32 {
        let mut steps = 0u32;
        let mut current_node = &self.nodes["AAA"];
        let mut path_position = 0usize;

        while current_node.name != "ZZZ" {
            current_node = match self
                .path
                .chars()
                .nth(path_position)
                .expect("Invalid path position")
            {
                'L' => &self.nodes[&current_node.left],
                'R' => &self.nodes[&current_node.right],
                _ => panic!("Unexpected direction in path"),
            };
            path_position = (path_position + 1) % self.path.len();
            steps += 1;
        }
        steps
    }

    fn get_num_steps_to_z_simultanious(&self) -> u64 {
        let mut steps = 0u64;
        let mut current_nodes =
            Vec::from_iter(self.nodes.values().filter(|n| n.name.ends_with("A")));
        let mut path_position = 0usize;
        let mut nodes_steps_to_z: Vec<u64> = Vec::new();

        while current_nodes.len() > 0 {
            let mut next_nodes = Vec::new();
            let direction = self
                .path
                .chars()
                .nth(path_position)
                .expect("Invalid path position");

            for current_node in current_nodes {
                if current_node.name.ends_with("Z") {
                    nodes_steps_to_z.push(steps);
                } else {
                    next_nodes.push(match direction {
                        'L' => &self.nodes[&current_node.left],
                        'R' => &self.nodes[&current_node.right],
                        _ => panic!("Unexpected direction in path"),
                    });
                }
            }
            current_nodes = next_nodes;
            path_position = (path_position + 1) % self.path.len();
            steps += 1;
        }
        nodes_steps_to_z.iter().cloned().reduce(|a, b| lcm(a, b)).unwrap()
    }
}

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, nodes) = s.split_once("\n\n").unwrap();
        let node_regex = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();
        let nodes = nodes.lines().map(|l| {
            let captures = node_regex
                .captures(l)
                .expect(format!("Invalid line {:?}", l).as_str());
            let (name, left, right) = (
                captures[1].to_string(),
                captures[2].to_string(),
                captures[3].to_string(),
            );
            Node { left, right, name }
        });

        let nodes_map = HashMap::from_iter(nodes.map(|n| (n.name.clone(), n)));

        Ok(Self {
            nodes: nodes_map,
            path: path.to_string(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let network = input.parse::<Network>().unwrap();
    Some(network.get_num_steps_to_zzz())
}

pub fn part_two(input: &str) -> Option<u64> {
    let network = input.parse::<Network>().unwrap();
    Some(network.get_num_steps_to_z_simultanious())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
