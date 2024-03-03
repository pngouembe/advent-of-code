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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: String,
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: String, bid: u64) -> Self {
        let mut counts = HashMap::new();

        for card in cards.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let duplicates = counts
            .values()
            .filter(|&&v| v >= 2)
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        let hand_type = match duplicates.len() {
            0 => HandType::HighCard,
            1 => match duplicates[0] {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => HandType::ThreeOfAKind,
                2 => HandType::OnePair,
                _ => panic!("invalid mono label duplicate count"),
            },
            2 => match duplicates[0] {
                3 => HandType::FullHouse,
                _ => HandType::TwoPair,
            },
            _ => panic!("Invalid hand"),
        };

        Hand {
            cards,
            bid,
            hand_type,
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
        let comparison_result = self.hand_type.cmp(&other.hand_type);

        if comparison_result != std::cmp::Ordering::Equal {
            return comparison_result;
        }

        for (self_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
            let self_card = get_value_from_card(self_card);
            let other_card = get_value_from_card(other_card);

            if self_card != other_card {
                return self_card.cmp(&other_card);
            }
        }

        panic!("Could not compare hands")
    }
}

fn get_value_from_card(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
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
            println!("Rank: {:}, Hand: {:?}", i + 1, hand);
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
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
