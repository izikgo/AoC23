use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Number(u32),
    Ten,
    // Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T" => Ok(Card::Ten),
            // "J" => Ok(Card::Jack),
            "J" => Ok(Card::Joker),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Ok(Card::Number(s.parse().unwrap())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn analyze_hand_type(cards: &[Card]) -> HandType {
        let mut card_counts = [0; 15];
        let mut num_jokers = 0;
        for card in cards {
            match card {
                Card::Number(n) => card_counts[*n as usize - 1] += 1,
                Card::Ten => card_counts[9] += 1,
                // Card::Jack => card_counts[10] += 1,
                Card::Joker => num_jokers += 1,
                Card::Queen => card_counts[11] += 1,
                Card::King => card_counts[12] += 1,
                Card::Ace => card_counts[13] += 1,
            }
        }

        if num_jokers == 5 {
            return HandType::FiveOfAKind;
        }

        let mut card_counts = card_counts.to_vec();
        card_counts.sort_unstable();

        // eliminate 0s
        while card_counts[0] == 0 {
            card_counts.remove(0);
        }

        // add jokers to max value
        let num_diff_cards = card_counts.len();
        card_counts[num_diff_cards - 1] += num_jokers;

        match card_counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(" ").unwrap();
        let mut cards = [Card::Number(0); 5];
        for (i, card) in hand.chars().enumerate() {
            cards[i] = card.to_string().parse().unwrap();
        }

        Ok(Self {
            hand_type: Self::analyze_hand_type(&cards),
            cards,
            bid: bid.parse().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.parse::<Hand>().unwrap())
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.parse::<Hand>().unwrap())
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
