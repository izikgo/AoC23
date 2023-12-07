use std::{collections::HashSet, str::FromStr};

use regex::Regex;

advent_of_code::solution!(4);

struct Card {
    winning_numbers: HashSet<u32>,
    owned_numbers: HashSet<u32>,
}

impl Card {
    fn new(winning_numbers: Vec<u32>, owned_numbers: Vec<u32>) -> Self {
        Self {
            winning_numbers: winning_numbers.into_iter().collect(),
            owned_numbers: owned_numbers.into_iter().collect(),
        }
    }

    fn num_hits(&self) -> usize {
        self.winning_numbers
            .intersection(&self.owned_numbers)
            .count()
    }

    fn get_score(&self) -> u32 {
        let num_hits = self.num_hits();
        // 2 power num_hits
        if num_hits == 0 {
            return 0;
        }
        2u32.pow(num_hits as u32 - 1)
    }
}

struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    fn get_score(&self) -> u32 {
        self.cards.iter().map(|x| x.get_score()).sum()
    }

    fn get_scores_new_rules(&self) -> u32 {
        let mut num_tickets_per_card = vec![1; self.cards.len()];

        for (i, card) in self.cards.iter().enumerate() {
            let num_hits = card.num_hits();
            let num_copies = num_tickets_per_card[i];

            num_tickets_per_card[i + 1..i + num_hits + 1]
                .iter_mut()
                .for_each(|x| *x += num_copies);
        }

        num_tickets_per_card.iter().sum()
    }
}

impl FromStr for Pile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Card +\d+\:([0-9 ]+)\|([0-9 ]+)").unwrap();
        let mut cards: Vec<Card> = Vec::new();
        let lines = s.lines();
        for line in lines {
            let captures = re.captures(line).unwrap();
            let winning_numbers: Vec<u32> = captures[1]
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let owned_numbers: Vec<u32> = captures[2]
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            cards.push(Card::new(winning_numbers, owned_numbers));
        }
        Ok(Self { cards })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pile = Pile::from_str(input).unwrap();
    Some(pile.get_score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let pile = Pile::from_str(input).unwrap();
    Some(pile.get_scores_new_rules())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
