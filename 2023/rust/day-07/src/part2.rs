use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::is_a,
    character::complete::{line_ending, space1, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    HighCard(Vec<char>),
    OnePair(char, Vec<char>),
    TwoPair(char, char, char),
    ThreeOfAKind(char, Vec<char>),
    FullHouse(char, char),
    FourOfAKind(char, char),
    FiveOfAKind(char),
}

impl HandType {
    fn get_score(&self) -> u32 {
        match self {
            HandType::HighCard(_) => 1,
            HandType::OnePair(_, _) => 2,
            HandType::TwoPair(_, _, _) => 3,
            HandType::ThreeOfAKind(_, _) => 4,
            HandType::FullHouse(_, _) => 5,
            HandType::FourOfAKind(_, _) => 6,
            HandType::FiveOfAKind(_) => 7,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_score().partial_cmp(&other.get_score())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    pub cards: String,
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: String, bid: u64) -> Self {
        let hand_type = Hand::get_hand_type_from_cards(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn get_hand_type_from_cards(cards: &str) -> HandType {
        let mut counts = HashMap::new();

        for card in cards.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let duplicates = counts
            .into_iter()
            .filter(|(_, v)| *v >= 2)
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .rev()
            .collect::<Vec<_>>();

        let initial_type = match duplicates.len() {
            0 => HandType::HighCard(cards.chars().collect()),
            1 => match duplicates[0].1 {
                5 => HandType::FiveOfAKind(duplicates[0].0),
                4 => HandType::FourOfAKind(
                    duplicates[0].0,
                    cards.chars().find(|c| *c != duplicates[0].0).unwrap(),
                ),
                3 => HandType::ThreeOfAKind(
                    duplicates[0].0,
                    cards.chars().filter(|c| *c != duplicates[0].0).collect(),
                ),
                2 => HandType::OnePair(
                    duplicates[0].0,
                    cards.chars().filter(|c| *c != duplicates[0].0).collect(),
                ),
                _ => panic!("invalid mono label duplicate count"),
            },
            2 => match duplicates[0].1 {
                3 => HandType::FullHouse(duplicates[0].0, duplicates[1].0),
                _ => HandType::TwoPair(
                    duplicates[0].0,
                    duplicates[1].0,
                    cards
                        .chars()
                        .find(|c| *c != duplicates[0].0 && *c != duplicates[1].0)
                        .unwrap(),
                ),
            },
            _ => panic!("Invalid hand"),
        };

        match initial_type {
            HandType::HighCard(v) if v.contains(&'J') => HandType::OnePair(v[0], v[1..].to_vec()),
            HandType::OnePair(c, v) if v.contains(&'J') => {
                HandType::ThreeOfAKind(c, v.iter().filter(|&&c| c != 'J').copied().collect())
            }
            HandType::OnePair('J', v) => HandType::ThreeOfAKind(v[0], v[1..].to_vec()),
            HandType::TwoPair(c1, c2, 'J') => HandType::FullHouse(c1, c2),
            HandType::TwoPair(c1, 'J', c2) => HandType::FourOfAKind(c1, c2),
            HandType::TwoPair('J', c1, c2) => HandType::FourOfAKind(c1, c2),
            HandType::ThreeOfAKind(c, v) if v.contains(&'J') => {
                HandType::FourOfAKind(c, v.iter().find(|&&c| c != 'J').unwrap().to_owned())
            }
            HandType::ThreeOfAKind('J', v) => {
                HandType::FourOfAKind(v[0], v.iter().find(|&&c| c != 'J').unwrap().to_owned())
            }
            HandType::FullHouse(c1, 'J') => HandType::FiveOfAKind(c1),
            HandType::FullHouse('J', c2) => HandType::FiveOfAKind(c2),
            HandType::FourOfAKind(c, 'J') => HandType::FiveOfAKind(c),
            HandType::FourOfAKind('J', c) => HandType::FiveOfAKind(c),
            _ => initial_type,
        }
    }

    fn get_value_from_card(card: char) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let comparison_result = self.hand_type.partial_cmp(&other.hand_type).unwrap();

        if comparison_result != std::cmp::Ordering::Equal {
            return comparison_result;
        }

        for (self_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
            let self_card = Hand::get_value_from_card(self_card);
            let other_card = Hand::get_value_from_card(other_card);

            if self_card != other_card {
                return self_card.cmp(&other_card);
            }
        }

        panic!("Could not compare hands")
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_, hands) = parse_input(_input).unwrap();

    let winnings: u64 = hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| {
            println!("Rank: {:}, Hand: {:?}", i + 1, hand.cards);
            hand.bid * (i as u64 + 1)
        })
        .sum();
    Ok(winnings.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(line_ending, parse_hand)(input)?;
    Ok((input, hands))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(is_a("AKQJT98765432"), space1, u64)(input)?;
    Ok((input, Hand::new(cards.to_string(), bid)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }

    #[test]
    fn test_comparison() -> miette::Result<()> {
        let input = "JJJJJ 1
AAAJJ 2
AT3K7 3
K2645 4";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
